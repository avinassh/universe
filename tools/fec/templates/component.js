class {{class_name}} extends HTMLElement {
    constructor() {
          super();
          this.shadow = this.attachShadow({mode:'open'});
          this.shadow.innerHTML = `<style>{{css}}</style>`

          this.props = [];
          this.state = [];
          this.stateMappers = {};

          this.__mappings = {};
          this.__selectors = {};

          this.initialize();
    }

    initialize() {
      this.setState = (newState) => {
        for (const k of Object.keys(newState)) {
          this.state[k] = newState[k];
          this.triggerMappings(k);
          this.triggerRenders("this.state." + k);
        }
      }

      {{javascript}}

      {{html}}

      this.render = (keys) => {
        for (const k of keys) {
          switch(k) {
            {{mutations[]}}
            case {{idx}}:
              {{code}}
              break;
            {{/mutations}}
            default:
              break;
          }
        }
      }

      this.initializeStateMappers();
    }

    initializeStateMappers() {
      for (const k of Object.keys(this.stateMappers)) {
        const args = getArguments(this.stateMappers[k]);
        for(const arg of args) {
          if(!this.__mappings[arg]) {
            this.__mappings[arg] = [];
          }
          this.__mappings[arg].push(k);
        }

        this.__selectors[k] = (state) => {
          let output = [];
          for(const arg of args) {
            output.push(state[arg]);
          }
          return output;
        }
      }
    }

    render(keys) {}

    triggerMappings(key) {
      if(!this.__mappings[key]) {
        return;
      }

      for(const m of this.__mappings[key]) {
        this.setState({
          [m]: this.stateMappers[m](...this.__selectors[m](this.state))
        })
      }
    }

    triggerRenders(key) {
      switch(key) {
        {{symbols[]}}
        case '{{name}}':
           this.render([{{mutations}}]);
           break;
        {{/symbols}}
        default:
          break;
      }
    }

    
    static get observedAttributes() {
      return [];
    }
}

customElements.define('{{component_name}}', {{class_name}});

function getArguments(func) {
    const ARROW = true;
    const FUNC_ARGS = ARROW ? /^(function)?\s*[^\(]*\(\s*([^\)]*)\)/m : /^(function)\s*[^\(]*\(\s*([^\)]*)\)/m;
    const FUNC_ARG_SPLIT = /,/;
    const FUNC_ARG = /^\s*(_?)(.+?)\1\s*$/;
    const STRIP_COMMENTS = /((\/\/.*$)|(\/\*[\s\S]*?\*\/))/mg;

    return ((func || '').toString().replace(STRIP_COMMENTS, '').match(FUNC_ARGS) || ['', '', ''])[2]
        .split(FUNC_ARG_SPLIT)
        .map(function(arg) {
            return arg.replace(FUNC_ARG, function(all, underscore, name) {
                return name.split('=')[0].trim();
            });
        })
        .filter(String);
}

