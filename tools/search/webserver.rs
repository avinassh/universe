use std::sync::Arc;

use crate::render;
use ws::{Body, Request, Response, Server};

static TEMPLATE: &str = include_str!("html/template.html");
static SIDEBAR: &str = include_str!("html/sidebar.html");
static INDEX: &str = include_str!("html/index.html");
static DETAIL: &str = include_str!("html/detail.html");
static DETAIL_MD: &str = include_str!("html/markdown.html");
static DETAIL_FOLDER: &str = include_str!("html/detail_folder.html");
static DETAIL_TEMPLATE: &str = include_str!("html/detail_template.html");
static RESULTS: &str = include_str!("html/results.html");

#[derive(Clone)]
pub struct SearchWebserver<A> {
    static_dir: String,
    auth: A,
    searcher: Arc<search_lib::Searcher>,
    base_url: String,
    settings: tmpl::ContentsMap,
}

impl<A> SearchWebserver<A>
where
    A: auth_client::AuthServer,
{
    pub fn new(
        searcher: Arc<search_lib::Searcher>,
        static_dir: String,
        base_url: String,
        auth: A,
        js_src: String,
    ) -> Self {
        Self {
            static_dir: static_dir,
            auth: auth,
            base_url: base_url,
            searcher: searcher,
            settings: content!("js_src" => js_src),
        }
    }

    fn wrap_template(&self, header: bool, query: &str, content: String) -> String {
        tmpl::apply_with_settings(
            TEMPLATE,
            content!(
                "title" => "code search",
                "show_header" => header,
                "query" => query,
                "content" => content),
            &self.settings,
        )
    }

    fn results(&self, keywords: &str, path: String, req: Request) -> Response {
        let mut results = self.searcher.search(keywords);

        if results.get_candidates().len() == 1 {
            // Only one search result! Skip right to the detail page.
            let mut response = Response::new(Body::from(""));
            self.redirect(
                &format!(
                    "/{}?q={}#L{}",
                    results.get_candidates()[0].get_filename(),
                    ws_utils::urlencode(keywords),
                    results.get_candidates()[0].get_jump_to_line() + 1,
                ),
                &mut response,
            );
            return response;
        }

        let page = tmpl::apply_with_settings(
            RESULTS,
            content!(
                "query" => keywords;
                "results" => results.get_candidates().iter().map(|r| render::result(r)).collect(),
                "languages" => results.take_languages().iter().map(|x| content!("name" => x)).collect(),
                "prefixes" => results.take_prefixes().iter().map(|x| content!("name" => x)).collect()
            ),
            &self.settings,
        );
        Response::new(Body::from(self.wrap_template(true, keywords, page)))
    }

    fn suggest(&self, query: &str, req: Request) -> Response {
        let mut response = self.searcher.suggest(query);
        let mut output = Vec::new();
        for keyword in response.take_suggestions().into_iter() {
            output.push(keyword);
        }

        Response::new(Body::from(json::stringify(output)))
    }

    fn detail(&self, query: &str, path: String, req: Request) -> Response {
        let file = match self.searcher.get_document(&path[1..]) {
            Some(f) => f,
            None => return self.not_found(path, req),
        };

        println!("pagerank: {:?}", file.get_page_rank());

        let sidebar = match path[1..].rmatch_indices("/").next() {
            Some((idx, _)) => match self.searcher.get_document(&path[1..idx + 1]) {
                Some(f) => tmpl::apply(
                    SIDEBAR,
                    &content!(
                            "parent_dir" => &path[1..idx+1],
                            "current_filename" => &path[idx+1..];
                            "sibling_directories" => f.get_child_directories().iter().map(|s| content!("child" => s, "selected" => s == &path[idx+2..])).collect(),
                            "sibling_files" => f.get_child_files().iter().map(|s| content!("child" => s, "selected" => s == &path[idx+2..])).collect()
                    ),
                ),
                None => {
                    println!("no document for for {}", &path[0..idx + 1]);
                    String::new()
                }
            },
            None => {
                println!("no match for {}", &path[1..]);
                String::new()
            }
        };

        let details = if file.get_is_directory() {
            tmpl::apply(DETAIL_FOLDER, &render::file(&file))
        } else if file.get_filename().ends_with(".md") {
            tmpl::apply(
                DETAIL_MD,
                &content!(
                    "markdown" => &markdown::to_html(&file.get_content())
                ),
            )
        } else {
            tmpl::apply_with_settings(DETAIL, render::file(&file), &self.settings)
        };

        let mut filename_components = Vec::new();
        let mut prev_idx = 0;
        for (idx, component) in file.get_filename().match_indices("/") {
            filename_components.push(content!(
                    "path" => file.get_filename()[0..idx].to_string(),
                    "section" => file.get_filename()[prev_idx..idx].to_string()
            ));
            prev_idx = idx;
        }
        filename_components.push(content!(
                "path" => file.get_filename().to_string(),
                "section" => file.get_filename()[prev_idx..].to_string()
        ));

        let page = tmpl::apply_with_settings(
            DETAIL_TEMPLATE,
            content!(
                "filename" => file.get_filename(),
                "sidebar" => sidebar,
                "detail" => details;

                // Extract the filename into clickable components
                "filename_components" => filename_components
            ),
            &self.settings,
        );

        Response::new(Body::from(self.wrap_template(true, query, page)))
    }

    fn index(&self, path: String, req: Request) -> Response {
        let page = tmpl::apply_with_settings(INDEX, content!(), &self.settings);
        Response::new(Body::from(self.wrap_template(false, "", page)))
    }

    fn not_found(&self, path: String, _req: Request) -> Response {
        Response::new(Body::from(format!("404 not found: path {}", path)))
    }
}

impl<A> Server for SearchWebserver<A>
where
    A: auth_client::AuthServer,
{
    fn respond(&self, path: String, req: Request, token: &str) -> Response {
        if path.starts_with("/static/") {
            return self.serve_static_files(path, "/static/", &self.static_dir);
        }

        let result = self.auth.authenticate(token.to_owned());
        if !result.get_success() {
            let challenge = self
                .auth
                .login_then_redirect(format!("{}{}", self.base_url, path));
            let mut response = Response::new(Body::from("redirect to login"));
            self.redirect(challenge.get_url(), &mut response);
            return response;
        }

        let mut query = String::new();
        if let Some(q) = req.uri().query() {
            let params = ws_utils::parse_params(q);
            if let Some(keywords) = params.get("q") {
                // Chrome's search engine plugin turns + into space
                query = keywords.replace("+", " ");
            }
        };

        if path == "/suggest" {
            return self.suggest(&query, req);
        }

        if path.len() > 1 {
            return self.detail(&query, path, req);
        }

        if query.len() > 0 {
            return self.results(&query, path, req);
        }

        self.index(path, req)
    }
}
