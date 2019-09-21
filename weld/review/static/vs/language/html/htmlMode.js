/*!-----------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * monaco-html version: 2.5.3(a819bbcf4554af1b8509c1ba5708fffed644f702)
 * Released under the MIT license
 * https://github.com/Microsoft/monaco-html/blob/master/LICENSE.md
 *-----------------------------------------------------------------------------*/
define("vs/language/html/workerManager",["require","exports"],function(e,t){"use strict";Object.defineProperty(t,"__esModule",{value:!0});var n=function(){function e(e){var t=this;this._defaults=e,this._worker=null,this._idleCheckInterval=setInterval(function(){return t._checkIfIdle()},3e4),this._lastUsedTime=0,this._configChangeListener=this._defaults.onDidChange(function(){return t._stopWorker()})}return e.prototype._stopWorker=function(){this._worker&&(this._worker.dispose(),this._worker=null),this._client=null},e.prototype.dispose=function(){clearInterval(this._idleCheckInterval),this._configChangeListener.dispose(),this._stopWorker()},e.prototype._checkIfIdle=function(){this._worker&&(12e4<Date.now()-this._lastUsedTime&&this._stopWorker())},e.prototype._getClient=function(){return this._lastUsedTime=Date.now(),this._client||(this._worker=monaco.editor.createWebWorker({moduleId:"vs/language/html/htmlWorker",createData:{languageSettings:this._defaults.options,languageId:this._defaults.languageId},label:this._defaults.languageId}),this._client=this._worker.getProxy()),this._client},e.prototype.getLanguageServiceWorker=function(){for(var t,n=this,r=[],e=0;e<arguments.length;e++)r[e]=arguments[e];return this._getClient().then(function(e){t=e}).then(function(e){return n._worker.withSyncedResources(r)}).then(function(e){return t})},e}();t.WorkerManager=n}),function(e){if("object"==typeof module&&"object"==typeof module.exports){var t=e(require,exports);void 0!==t&&(module.exports=t)}else"function"==typeof define&&define.amd&&define("vscode-languageserver-types/main",["require","exports"],e)}(function(e,t){"use strict";var a,n,r,i,o,u,s,c,d,l,g,f,m,p,h,v,y,b,k,C,_,I,x,w,S,E,D,K,F,T,M;Object.defineProperty(t,"__esModule",{value:!0}),(n=a=t.Position||(t.Position={})).create=function(e,t){return{line:e,character:t}},n.is=function(e){var t=e;return ae.objectLiteral(t)&&ae.number(t.line)&&ae.number(t.character)},(i=r=t.Range||(t.Range={})).create=function(e,t,n,r){if(ae.number(e)&&ae.number(t)&&ae.number(n)&&ae.number(r))return{start:a.create(e,t),end:a.create(n,r)};if(a.is(e)&&a.is(t))return{start:e,end:t};throw new Error("Range#create called with invalid arguments["+e+", "+t+", "+n+", "+r+"]")},i.is=function(e){var t=e;return ae.objectLiteral(t)&&a.is(t.start)&&a.is(t.end)},(u=o=t.Location||(t.Location={})).create=function(e,t){return{uri:e,range:t}},u.is=function(e){var t=e;return ae.defined(t)&&r.is(t.range)&&(ae.string(t.uri)||ae.undefined(t.uri))},(s=t.LocationLink||(t.LocationLink={})).create=function(e,t,n,r){return{targetUri:e,targetRange:t,targetSelectionRange:n,originSelectionRange:r}},s.is=function(e){var t=e;return ae.defined(t)&&r.is(t.targetRange)&&ae.string(t.targetUri)&&(r.is(t.targetSelectionRange)||ae.undefined(t.targetSelectionRange))&&(r.is(t.originSelectionRange)||ae.undefined(t.originSelectionRange))},(d=c=t.Color||(t.Color={})).create=function(e,t,n,r){return{red:e,green:t,blue:n,alpha:r}},d.is=function(e){var t=e;return ae.number(t.red)&&ae.number(t.green)&&ae.number(t.blue)&&ae.number(t.alpha)},(l=t.ColorInformation||(t.ColorInformation={})).create=function(e,t){return{range:e,color:t}},l.is=function(e){var t=e;return r.is(t.range)&&c.is(t.color)},(g=t.ColorPresentation||(t.ColorPresentation={})).create=function(e,t,n){return{label:e,textEdit:t,additionalTextEdits:n}},g.is=function(e){var t=e;return ae.string(t.label)&&(ae.undefined(t.textEdit)||_.is(t))&&(ae.undefined(t.additionalTextEdits)||ae.typedArray(t.additionalTextEdits,_.is))},(f=t.FoldingRangeKind||(t.FoldingRangeKind={})).Comment="comment",f.Imports="imports",f.Region="region",(m=t.FoldingRange||(t.FoldingRange={})).create=function(e,t,n,r,i){var o={startLine:e,endLine:t};return ae.defined(n)&&(o.startCharacter=n),ae.defined(r)&&(o.endCharacter=r),ae.defined(i)&&(o.kind=i),o},m.is=function(e){var t=e;return ae.number(t.startLine)&&ae.number(t.startLine)&&(ae.undefined(t.startCharacter)||ae.number(t.startCharacter))&&(ae.undefined(t.endCharacter)||ae.number(t.endCharacter))&&(ae.undefined(t.kind)||ae.string(t.kind))},(h=p=t.DiagnosticRelatedInformation||(t.DiagnosticRelatedInformation={})).create=function(e,t){return{location:e,message:t}},h.is=function(e){var t=e;return ae.defined(t)&&o.is(t.location)&&ae.string(t.message)},(v=t.DiagnosticSeverity||(t.DiagnosticSeverity={})).Error=1,v.Warning=2,v.Information=3,v.Hint=4,(b=y=t.Diagnostic||(t.Diagnostic={})).create=function(e,t,n,r,i,o){var a={range:e,message:t};return ae.defined(n)&&(a.severity=n),ae.defined(r)&&(a.code=r),ae.defined(i)&&(a.source=i),ae.defined(o)&&(a.relatedInformation=o),a},b.is=function(e){var t=e;return ae.defined(t)&&r.is(t.range)&&ae.string(t.message)&&(ae.number(t.severity)||ae.undefined(t.severity))&&(ae.number(t.code)||ae.string(t.code)||ae.undefined(t.code))&&(ae.string(t.source)||ae.undefined(t.source))&&(ae.undefined(t.relatedInformation)||ae.typedArray(t.relatedInformation,p.is))},(C=k=t.Command||(t.Command={})).create=function(e,t){for(var n=[],r=2;r<arguments.length;r++)n[r-2]=arguments[r];var i={title:e,command:t};return ae.defined(n)&&0<n.length&&(i.arguments=n),i},C.is=function(e){var t=e;return ae.defined(t)&&ae.string(t.title)&&ae.string(t.command)},(I=_=t.TextEdit||(t.TextEdit={})).replace=function(e,t){return{range:e,newText:t}},I.insert=function(e,t){return{range:{start:e,end:e},newText:t}},I.del=function(e){return{range:e,newText:""}},I.is=function(e){var t=e;return ae.objectLiteral(t)&&ae.string(t.newText)&&r.is(t.range)},(w=x=t.TextDocumentEdit||(t.TextDocumentEdit={})).create=function(e,t){return{textDocument:e,edits:t}},w.is=function(e){var t=e;return ae.defined(t)&&A.is(t.textDocument)&&Array.isArray(t.edits)},(E=S=t.CreateFile||(t.CreateFile={})).create=function(e,t){var n={kind:"create",uri:e};return void 0===t||void 0===t.overwrite&&void 0===t.ignoreIfExists||(n.options=t),n},E.is=function(e){var t=e;return t&&"create"===t.kind&&ae.string(t.uri)&&(void 0===t.options||(void 0===t.options.overwrite||ae.boolean(t.options.overwrite))&&(void 0===t.options.ignoreIfExists||ae.boolean(t.options.ignoreIfExists)))},(K=D=t.RenameFile||(t.RenameFile={})).create=function(e,t,n){var r={kind:"rename",oldUri:e,newUri:t};return void 0===n||void 0===n.overwrite&&void 0===n.ignoreIfExists||(r.options=n),r},K.is=function(e){var t=e;return t&&"rename"===t.kind&&ae.string(t.oldUri)&&ae.string(t.newUri)&&(void 0===t.options||(void 0===t.options.overwrite||ae.boolean(t.options.overwrite))&&(void 0===t.options.ignoreIfExists||ae.boolean(t.options.ignoreIfExists)))},(T=F=t.DeleteFile||(t.DeleteFile={})).create=function(e,t){var n={kind:"delete",uri:e};return void 0===t||void 0===t.recursive&&void 0===t.ignoreIfNotExists||(n.options=t),n},T.is=function(e){var t=e;return t&&"delete"===t.kind&&ae.string(t.uri)&&(void 0===t.options||(void 0===t.options.recursive||ae.boolean(t.options.recursive))&&(void 0===t.options.ignoreIfNotExists||ae.boolean(t.options.ignoreIfNotExists)))},(M=t.WorkspaceEdit||(t.WorkspaceEdit={})).is=function(e){var t=e;return t&&(void 0!==t.changes||void 0!==t.documentChanges)&&(void 0===t.documentChanges||t.documentChanges.every(function(e){return ae.string(e.kind)?S.is(e)||D.is(e)||F.is(e):x.is(e)}))};var R,A,P,L,O,j,W,H,N,V,U,q,z,B,$=function(){function e(e){this.edits=e}return e.prototype.insert=function(e,t){this.edits.push(_.insert(e,t))},e.prototype.replace=function(e,t){this.edits.push(_.replace(e,t))},e.prototype.delete=function(e){this.edits.push(_.del(e))},e.prototype.add=function(e){this.edits.push(e)},e.prototype.all=function(){return this.edits},e.prototype.clear=function(){this.edits.splice(0,this.edits.length)},e}(),Q=function(){function e(n){var r=this;this._textEditChanges=Object.create(null),n&&((this._workspaceEdit=n).documentChanges?n.documentChanges.forEach(function(e){if(x.is(e)){var t=new $(e.edits);r._textEditChanges[e.textDocument.uri]=t}}):n.changes&&Object.keys(n.changes).forEach(function(e){var t=new $(n.changes[e]);r._textEditChanges[e]=t}))}return Object.defineProperty(e.prototype,"edit",{get:function(){return this._workspaceEdit},enumerable:!0,configurable:!0}),e.prototype.getTextEditChange=function(e){if(A.is(e)){if(this._workspaceEdit||(this._workspaceEdit={documentChanges:[]}),!this._workspaceEdit.documentChanges)throw new Error("Workspace edit is not configured for document changes.");var t=e;if(!(r=this._textEditChanges[t.uri])){var n={textDocument:t,edits:i=[]};this._workspaceEdit.documentChanges.push(n),r=new $(i),this._textEditChanges[t.uri]=r}return r}if(this._workspaceEdit||(this._workspaceEdit={changes:Object.create(null)}),!this._workspaceEdit.changes)throw new Error("Workspace edit is not configured for normal text edit changes.");var r;if(!(r=this._textEditChanges[e])){var i=[];this._workspaceEdit.changes[e]=i,r=new $(i),this._textEditChanges[e]=r}return r},e.prototype.createFile=function(e,t){this.checkDocumentChanges(),this._workspaceEdit.documentChanges.push(S.create(e,t))},e.prototype.renameFile=function(e,t,n){this.checkDocumentChanges(),this._workspaceEdit.documentChanges.push(D.create(e,t,n))},e.prototype.deleteFile=function(e,t){this.checkDocumentChanges(),this._workspaceEdit.documentChanges.push(F.create(e,t))},e.prototype.checkDocumentChanges=function(){if(!this._workspaceEdit||!this._workspaceEdit.documentChanges)throw new Error("Workspace edit is not configured for document changes.")},e}();t.WorkspaceChange=Q,(R=t.TextDocumentIdentifier||(t.TextDocumentIdentifier={})).create=function(e){return{uri:e}},R.is=function(e){var t=e;return ae.defined(t)&&ae.string(t.uri)},(P=A=t.VersionedTextDocumentIdentifier||(t.VersionedTextDocumentIdentifier={})).create=function(e,t){return{uri:e,version:t}},P.is=function(e){var t=e;return ae.defined(t)&&ae.string(t.uri)&&(null===t.version||ae.number(t.version))},(L=t.TextDocumentItem||(t.TextDocumentItem={})).create=function(e,t,n,r){return{uri:e,languageId:t,version:n,text:r}},L.is=function(e){var t=e;return ae.defined(t)&&ae.string(t.uri)&&ae.string(t.languageId)&&ae.number(t.version)&&ae.string(t.text)},(j=O=t.MarkupKind||(t.MarkupKind={})).PlainText="plaintext",j.Markdown="markdown",(W=O=t.MarkupKind||(t.MarkupKind={})).is=function(e){var t=e;return t===W.PlainText||t===W.Markdown},(H=t.MarkupContent||(t.MarkupContent={})).is=function(e){var t=e;return ae.objectLiteral(e)&&O.is(t.kind)&&ae.string(t.value)},(N=t.CompletionItemKind||(t.CompletionItemKind={})).Text=1,N.Method=2,N.Function=3,N.Constructor=4,N.Field=5,N.Variable=6,N.Class=7,N.Interface=8,N.Module=9,N.Property=10,N.Unit=11,N.Value=12,N.Enum=13,N.Keyword=14,N.Snippet=15,N.Color=16,N.File=17,N.Reference=18,N.Folder=19,N.EnumMember=20,N.Constant=21,N.Struct=22,N.Event=23,N.Operator=24,N.TypeParameter=25,(V=t.InsertTextFormat||(t.InsertTextFormat={})).PlainText=1,V.Snippet=2,(t.CompletionItem||(t.CompletionItem={})).create=function(e){return{label:e}},(t.CompletionList||(t.CompletionList={})).create=function(e,t){return{items:e||[],isIncomplete:!!t}},(q=U=t.MarkedString||(t.MarkedString={})).fromPlainText=function(e){return e.replace(/[\\`*_{}[\]()#+\-.!]/g,"\\$&")},q.is=function(e){var t=e;return ae.string(t)||ae.objectLiteral(t)&&ae.string(t.language)&&ae.string(t.value)},(t.Hover||(t.Hover={})).is=function(e){var t=e;return!!t&&ae.objectLiteral(t)&&(H.is(t.contents)||U.is(t.contents)||ae.typedArray(t.contents,U.is))&&(void 0===e.range||r.is(e.range))},(t.ParameterInformation||(t.ParameterInformation={})).create=function(e,t){return t?{label:e,documentation:t}:{label:e}},(t.SignatureInformation||(t.SignatureInformation={})).create=function(e,t){for(var n=[],r=2;r<arguments.length;r++)n[r-2]=arguments[r];var i={label:e};return ae.defined(t)&&(i.documentation=t),ae.defined(n)?i.parameters=n:i.parameters=[],i},(z=t.DocumentHighlightKind||(t.DocumentHighlightKind={})).Text=1,z.Read=2,z.Write=3,(t.DocumentHighlight||(t.DocumentHighlight={})).create=function(e,t){var n={range:e};return ae.number(t)&&(n.kind=t),n},(B=t.SymbolKind||(t.SymbolKind={})).File=1,B.Module=2,B.Namespace=3,B.Package=4,B.Class=5,B.Method=6,B.Property=7,B.Field=8,B.Constructor=9,B.Enum=10,B.Interface=11,B.Function=12,B.Variable=13,B.Constant=14,B.String=15,B.Number=16,B.Boolean=17,B.Array=18,B.Object=19,B.Key=20,B.Null=21,B.EnumMember=22,B.Struct=23,B.Event=24,B.Operator=25,B.TypeParameter=26,(t.SymbolInformation||(t.SymbolInformation={})).create=function(e,t,n,r,i){var o={name:e,kind:t,location:{uri:r,range:n}};return i&&(o.containerName=i),o};var G,J,X,Y,Z,ee,te=function(){};t.DocumentSymbol=te,(G=te=t.DocumentSymbol||(t.DocumentSymbol={})).create=function(e,t,n,r,i,o){var a={name:e,detail:t,kind:n,range:r,selectionRange:i};return void 0!==o&&(a.children=o),a},G.is=function(e){var t=e;return t&&ae.string(t.name)&&ae.number(t.kind)&&r.is(t.range)&&r.is(t.selectionRange)&&(void 0===t.detail||ae.string(t.detail))&&(void 0===t.deprecated||ae.boolean(t.deprecated))&&(void 0===t.children||Array.isArray(t.children))},t.DocumentSymbol=te,(J=t.CodeActionKind||(t.CodeActionKind={})).QuickFix="quickfix",J.Refactor="refactor",J.RefactorExtract="refactor.extract",J.RefactorInline="refactor.inline",J.RefactorRewrite="refactor.rewrite",J.Source="source",J.SourceOrganizeImports="source.organizeImports",(X=t.CodeActionContext||(t.CodeActionContext={})).create=function(e,t){var n={diagnostics:e};return null!=t&&(n.only=t),n},X.is=function(e){var t=e;return ae.defined(t)&&ae.typedArray(t.diagnostics,y.is)&&(void 0===t.only||ae.typedArray(t.only,ae.string))},(Y=t.CodeAction||(t.CodeAction={})).create=function(e,t,n){var r={title:e};return k.is(t)?r.command=t:r.edit=t,void 0!==n&&(r.kind=n),r},Y.is=function(e){var t=e;return t&&ae.string(t.title)&&(void 0===t.diagnostics||ae.typedArray(t.diagnostics,y.is))&&(void 0===t.kind||ae.string(t.kind))&&(void 0!==t.edit||void 0!==t.command)&&(void 0===t.command||k.is(t.command))&&(void 0===t.edit||M.is(t.edit))},(Z=t.CodeLens||(t.CodeLens={})).create=function(e,t){var n={range:e};return ae.defined(t)&&(n.data=t),n},Z.is=function(e){var t=e;return ae.defined(t)&&r.is(t.range)&&(ae.undefined(t.command)||k.is(t.command))},(ee=t.FormattingOptions||(t.FormattingOptions={})).create=function(e,t){return{tabSize:e,insertSpaces:t}},ee.is=function(e){var t=e;return ae.defined(t)&&ae.number(t.tabSize)&&ae.boolean(t.insertSpaces)};var ne,re,ie,oe=function(){};t.DocumentLink=oe,(ne=oe=t.DocumentLink||(t.DocumentLink={})).create=function(e,t,n){return{range:e,target:t,data:n}},ne.is=function(e){var t=e;return ae.defined(t)&&r.is(t.range)&&(ae.undefined(t.target)||ae.string(t.target))},t.DocumentLink=oe,t.EOL=["\n","\r\n","\r"],(re=t.TextDocument||(t.TextDocument={})).create=function(e,t,n,r){return new ce(e,t,n,r)},re.is=function(e){var t=e;return!!(ae.defined(t)&&ae.string(t.uri)&&(ae.undefined(t.languageId)||ae.string(t.languageId))&&ae.number(t.lineCount)&&ae.func(t.getText)&&ae.func(t.positionAt)&&ae.func(t.offsetAt))},re.applyEdits=function(e,t){for(var n=e.getText(),r=function e(t,n){if(t.length<=1)return t;var r=t.length/2|0,i=t.slice(0,r),o=t.slice(r);e(i,n),e(o,n);for(var a=0,u=0,s=0;a<i.length&&u<o.length;){var c=n(i[a],o[u]);t[s++]=c<=0?i[a++]:o[u++]}for(;a<i.length;)t[s++]=i[a++];for(;u<o.length;)t[s++]=o[u++];return t}(t,function(e,t){var n=e.range.start.line-t.range.start.line;return 0===n?e.range.start.character-t.range.start.character:n}),i=n.length,o=r.length-1;0<=o;o--){var a=r[o],u=e.offsetAt(a.range.start),s=e.offsetAt(a.range.end);if(!(s<=i))throw new Error("Overlapping edit");n=n.substring(0,u)+a.newText+n.substring(s,n.length),i=u}return n},(ie=t.TextDocumentSaveReason||(t.TextDocumentSaveReason={})).Manual=1,ie.AfterDelay=2,ie.FocusOut=3;var ae,ue,se,ce=function(){function e(e,t,n,r){this._uri=e,this._languageId=t,this._version=n,this._content=r,this._lineOffsets=null}return Object.defineProperty(e.prototype,"uri",{get:function(){return this._uri},enumerable:!0,configurable:!0}),Object.defineProperty(e.prototype,"languageId",{get:function(){return this._languageId},enumerable:!0,configurable:!0}),Object.defineProperty(e.prototype,"version",{get:function(){return this._version},enumerable:!0,configurable:!0}),e.prototype.getText=function(e){if(e){var t=this.offsetAt(e.start),n=this.offsetAt(e.end);return this._content.substring(t,n)}return this._content},e.prototype.update=function(e,t){this._content=e.text,this._version=t,this._lineOffsets=null},e.prototype.getLineOffsets=function(){if(null===this._lineOffsets){for(var e=[],t=this._content,n=!0,r=0;r<t.length;r++){n&&(e.push(r),n=!1);var i=t.charAt(r);n="\r"===i||"\n"===i,"\r"===i&&r+1<t.length&&"\n"===t.charAt(r+1)&&r++}n&&0<t.length&&e.push(t.length),this._lineOffsets=e}return this._lineOffsets},e.prototype.positionAt=function(e){e=Math.max(Math.min(e,this._content.length),0);var t=this.getLineOffsets(),n=0,r=t.length;if(0===r)return a.create(0,e);for(;n<r;){var i=Math.floor((n+r)/2);t[i]>e?r=i:n=i+1}var o=n-1;return a.create(o,e-t[o])},e.prototype.offsetAt=function(e){var t=this.getLineOffsets();if(e.line>=t.length)return this._content.length;if(e.line<0)return 0;var n=t[e.line],r=e.line+1<t.length?t[e.line+1]:this._content.length;return Math.max(Math.min(n+e.character,r),n)},Object.defineProperty(e.prototype,"lineCount",{get:function(){return this.getLineOffsets().length},enumerable:!0,configurable:!0}),e}();ue=ae||(ae={}),se=Object.prototype.toString,ue.defined=function(e){return void 0!==e},ue.undefined=function(e){return void 0===e},ue.boolean=function(e){return!0===e||!1===e},ue.string=function(e){return"[object String]"===se.call(e)},ue.number=function(e){return"[object Number]"===se.call(e)},ue.func=function(e){return"[object Function]"===se.call(e)},ue.objectLiteral=function(e){return null!==e&&"object"==typeof e},ue.typedArray=function(e,t){return Array.isArray(e)&&e.every(t)}}),define("vscode-languageserver-types",["vscode-languageserver-types/main"],function(e){return e}),define("vs/language/html/languageFeatures",["require","exports","vscode-languageserver-types"],function(e,t,a){"use strict";Object.defineProperty(t,"__esModule",{value:!0});var u=monaco.Range,n=function(){function e(e,t,n){var r=this;this._languageId=e,this._worker=t,this._disposables=[],this._listener=Object.create(null);var i=function(e){var t,n=e.getModeId();n===r._languageId&&(r._listener[e.uri.toString()]=e.onDidChangeContent(function(){clearTimeout(t),t=setTimeout(function(){return r._doValidate(e.uri,n)},500)}),r._doValidate(e.uri,n))},o=function(e){monaco.editor.setModelMarkers(e,r._languageId,[]);var t=e.uri.toString(),n=r._listener[t];n&&(n.dispose(),delete r._listener[t])};this._disposables.push(monaco.editor.onDidCreateModel(i)),this._disposables.push(monaco.editor.onWillDisposeModel(function(e){o(e)})),this._disposables.push(monaco.editor.onDidChangeModelLanguage(function(e){o(e.model),i(e.model)})),this._disposables.push(n.onDidChange(function(e){monaco.editor.getModels().forEach(function(e){e.getModeId()===r._languageId&&(o(e),i(e))})})),this._disposables.push({dispose:function(){for(var e in r._listener)r._listener[e].dispose()}}),monaco.editor.getModels().forEach(i)}return e.prototype.dispose=function(){this._disposables.forEach(function(e){return e&&e.dispose()}),this._disposables=[]},e.prototype._doValidate=function(n,r){this._worker(n).then(function(e){return e.doValidation(n.toString()).then(function(e){var t=e.map(function(e){return n="number"==typeof(t=e).code?String(t.code):t.code,{severity:function(e){switch(e){case a.DiagnosticSeverity.Error:return monaco.MarkerSeverity.Error;case a.DiagnosticSeverity.Warning:return monaco.MarkerSeverity.Warning;case a.DiagnosticSeverity.Information:return monaco.MarkerSeverity.Info;case a.DiagnosticSeverity.Hint:return monaco.MarkerSeverity.Hint;default:return monaco.MarkerSeverity.Info}}(t.severity),startLineNumber:t.range.start.line+1,startColumn:t.range.start.character+1,endLineNumber:t.range.end.line+1,endColumn:t.range.end.character+1,message:t.message,code:n,source:t.source};var t,n});monaco.editor.setModelMarkers(monaco.editor.getModel(n),r,t)})}).then(void 0,function(e){console.error(e)})},e}();function s(e){if(e)return{character:e.column-1,line:e.lineNumber-1}}function c(e){if(e)return new u(e.start.line+1,e.start.character+1,e.end.line+1,e.end.character+1)}function d(e){if(e)return{range:c(e.range),text:e.newText}}t.DiagnosticsAdapter=n;var r=function(){function e(e){this._worker=e}return Object.defineProperty(e.prototype,"triggerCharacters",{get:function(){return[".",":","<",'"',"=","/"]},enumerable:!0,configurable:!0}),e.prototype.provideCompletionItems=function(i,o,e,t){var n=i.uri;return this._worker(n).then(function(e){return e.doComplete(n.toString(),s(o))}).then(function(e){if(e){var t=i.getWordUntilPosition(o),n=new u(o.lineNumber,t.startColumn,o.lineNumber,t.endColumn),r=e.items.map(function(e){var t={label:e.label,insertText:e.insertText||e.label,sortText:e.sortText,filterText:e.filterText,documentation:e.documentation,detail:e.detail,range:n,kind:function(e){var t=monaco.languages.CompletionItemKind;switch(e){case a.CompletionItemKind.Text:return t.Text;case a.CompletionItemKind.Method:return t.Method;case a.CompletionItemKind.Function:return t.Function;case a.CompletionItemKind.Constructor:return t.Constructor;case a.CompletionItemKind.Field:return t.Field;case a.CompletionItemKind.Variable:return t.Variable;case a.CompletionItemKind.Class:return t.Class;case a.CompletionItemKind.Interface:return t.Interface;case a.CompletionItemKind.Module:return t.Module;case a.CompletionItemKind.Property:return t.Property;case a.CompletionItemKind.Unit:return t.Unit;case a.CompletionItemKind.Value:return t.Value;case a.CompletionItemKind.Enum:return t.Enum;case a.CompletionItemKind.Keyword:return t.Keyword;case a.CompletionItemKind.Snippet:return t.Snippet;case a.CompletionItemKind.Color:return t.Color;case a.CompletionItemKind.File:return t.File;case a.CompletionItemKind.Reference:return t.Reference}return t.Property}(e.kind)};return e.textEdit&&(t.range=c(e.textEdit.range),t.insertText=e.textEdit.newText),e.additionalTextEdits&&(t.additionalTextEdits=e.additionalTextEdits.map(d)),e.insertTextFormat===a.InsertTextFormat.Snippet&&(t.insertTextRules=monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet),t});return{isIncomplete:e.isIncomplete,suggestions:r}}})},e}();function i(e){return"string"==typeof e?{value:e}:(t=e)&&"object"==typeof t&&"string"==typeof t.kind?"plaintext"===e.kind?{value:e.value.replace(/[\\`*_{}[\]()#+\-.!]/g,"\\$&")}:{value:e.value}:{value:"```"+e.language+"\n"+e.value+"\n```\n"};var t}t.CompletionAdapter=r;var o=function(){function e(e){this._worker=e}return e.prototype.provideHover=function(e,t,n){var r=e.uri;return this._worker(r).then(function(e){return e.doHover(r.toString(),s(t))}).then(function(e){if(e)return{range:c(e.range),contents:function(e){if(e)return Array.isArray(e)?e.map(i):[i(e)]}(e.contents)}})},e}();t.HoverAdapter=o;var l=function(){function e(e){this._worker=e}return e.prototype.provideDocumentHighlights=function(e,t,n){var r=e.uri;return this._worker(r).then(function(e){return e.findDocumentHighlights(r.toString(),s(t))}).then(function(e){if(e)return e.map(function(e){return{range:c(e.range),kind:function(e){var t=monaco.languages.DocumentHighlightKind;switch(e){case a.DocumentHighlightKind.Read:return t.Read;case a.DocumentHighlightKind.Write:return t.Write;case a.DocumentHighlightKind.Text:return t.Text}return t.Text}(e.kind)}})})},e}();t.DocumentHighlightAdapter=l;var g=function(){function e(e){this._worker=e}return e.prototype.provideDocumentSymbols=function(e,t){var n=e.uri;return this._worker(n).then(function(e){return e.findDocumentSymbols(n.toString())}).then(function(e){if(e)return e.map(function(e){return{name:e.name,detail:"",containerName:e.containerName,kind:function(e){var t=monaco.languages.SymbolKind;switch(e){case a.SymbolKind.File:return t.Array;case a.SymbolKind.Module:return t.Module;case a.SymbolKind.Namespace:return t.Namespace;case a.SymbolKind.Package:return t.Package;case a.SymbolKind.Class:return t.Class;case a.SymbolKind.Method:return t.Method;case a.SymbolKind.Property:return t.Property;case a.SymbolKind.Field:return t.Field;case a.SymbolKind.Constructor:return t.Constructor;case a.SymbolKind.Enum:return t.Enum;case a.SymbolKind.Interface:return t.Interface;case a.SymbolKind.Function:return t.Function;case a.SymbolKind.Variable:return t.Variable;case a.SymbolKind.Constant:return t.Constant;case a.SymbolKind.String:return t.String;case a.SymbolKind.Number:return t.Number;case a.SymbolKind.Boolean:return t.Boolean;case a.SymbolKind.Array:return t.Array}return t.Function}(e.kind),tags:[],range:c(e.location.range),selectionRange:c(e.location.range)}})})},e}();t.DocumentSymbolAdapter=g;var f=function(){function e(e){this._worker=e}return e.prototype.provideLinks=function(e,t){var n=e.uri;return this._worker(n).then(function(e){return e.findDocumentLinks(n.toString())}).then(function(e){if(e)return{links:e.map(function(e){return{range:c(e.range),url:e.target}})}})},e}();function m(e){return{tabSize:e.tabSize,insertSpaces:e.insertSpaces}}t.DocumentLinkAdapter=f;var p=function(){function e(e){this._worker=e}return e.prototype.provideDocumentFormattingEdits=function(e,t,n){var r=e.uri;return this._worker(r).then(function(e){return e.format(r.toString(),null,m(t)).then(function(e){if(e&&0!==e.length)return e.map(d)})})},e}();t.DocumentFormattingEditProvider=p;var h=function(){function e(e){this._worker=e}return e.prototype.provideDocumentRangeFormattingEdits=function(e,t,n,r){var i=e.uri;return this._worker(i).then(function(e){return e.format(i.toString(),function(e){if(e)return{start:s(e.getStartPosition()),end:s(e.getEndPosition())}}(t),m(n)).then(function(e){if(e&&0!==e.length)return e.map(d)})})},e}();t.DocumentRangeFormattingEditProvider=h;var v=function(){function e(e){this._worker=e}return e.prototype.provideFoldingRanges=function(e,t,n){var r=e.uri;return this._worker(r).then(function(e){return e.provideFoldingRanges(r.toString(),t)}).then(function(e){if(e)return e.map(function(e){var t={start:e.startLine+1,end:e.endLine+1};return void 0!==e.kind&&(t.kind=function(e){switch(e){case a.FoldingRangeKind.Comment:return monaco.languages.FoldingRangeKind.Comment;case a.FoldingRangeKind.Imports:return monaco.languages.FoldingRangeKind.Imports;case a.FoldingRangeKind.Region:return monaco.languages.FoldingRangeKind.Region}return}(e.kind)),t})})},e}();t.FoldingRangeAdapter=v}),define("vs/language/html/htmlMode",["require","exports","./workerManager","./languageFeatures"],function(e,t,i,o){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.setupMode=function(e){var n=new i.WorkerManager(e),t=function(){for(var e=[],t=0;t<arguments.length;t++)e[t]=arguments[t];return n.getLanguageServiceWorker.apply(n,e)},r=e.languageId;monaco.languages.registerCompletionItemProvider(r,new o.CompletionAdapter(t)),monaco.languages.registerHoverProvider(r,new o.HoverAdapter(t)),monaco.languages.registerDocumentHighlightProvider(r,new o.DocumentHighlightAdapter(t)),monaco.languages.registerLinkProvider(r,new o.DocumentLinkAdapter(t)),monaco.languages.registerFoldingRangeProvider(r,new o.FoldingRangeAdapter(t)),monaco.languages.registerDocumentSymbolProvider(r,new o.DocumentSymbolAdapter(t)),"html"===r&&(monaco.languages.registerDocumentFormattingEditProvider(r,new o.DocumentFormattingEditProvider(t)),monaco.languages.registerDocumentRangeFormattingEditProvider(r,new o.DocumentRangeFormattingEditProvider(t)),new o.DiagnosticsAdapter(r,t,e))}});