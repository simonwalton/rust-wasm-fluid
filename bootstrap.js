(()=>{var e,t,r,n,o,i,a,s,c,u,l={820:(e,t,r)=>{r.e(170).then(r.bind(r,170)).catch((e=>console.error("Error importing `index.js`:",e))),r.e(766).then(r.bind(r,766)).catch((e=>console.error("Error importing `config.js`:",e)))}},d={};function p(e){var t=d[e];if(void 0!==t)return t.exports;var r=d[e]={id:e,loaded:!1,exports:{}};return l[e](r,r.exports,p),r.loaded=!0,r.exports}p.m=l,p.c=d,p.d=(e,t)=>{for(var r in t)p.o(t,r)&&!p.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},p.f={},p.e=e=>Promise.all(Object.keys(p.f).reduce(((t,r)=>(p.f[r](e,t),t)),[])),p.u=e=>e+".bootstrap.js",p.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),p.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),p.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),e={},t="create-wasm-app:",p.l=(r,n,o,i)=>{if(e[r])e[r].push(n);else{var a,s;if(void 0!==o)for(var c=document.getElementsByTagName("script"),u=0;u<c.length;u++){var l=c[u];if(l.getAttribute("src")==r||l.getAttribute("data-webpack")==t+o){a=l;break}}a||(s=!0,(a=document.createElement("script")).charset="utf-8",a.timeout=120,p.nc&&a.setAttribute("nonce",p.nc),a.setAttribute("data-webpack",t+o),a.src=r),e[r]=[n];var d=(t,n)=>{a.onerror=a.onload=null,clearTimeout(f);var o=e[r];if(delete e[r],a.parentNode&&a.parentNode.removeChild(a),o&&o.forEach((e=>e(n))),t)return t(n)},f=setTimeout(d.bind(null,void 0,{type:"timeout",target:a}),12e4);a.onerror=d.bind(null,a.onerror),a.onload=d.bind(null,a.onload),s&&document.head.appendChild(a)}},p.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;p.g.importScripts&&(e=p.g.location+"");var t=p.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");if(r.length)for(var n=r.length-1;n>-1&&!e;)e=r[n--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),p.p=e})(),(()=>{var e={179:0};p.f.j=(t,r)=>{var n=p.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,o)=>n=e[t]=[r,o]));r.push(n[2]=o);var i=p.p+p.u(t),a=new Error;p.l(i,(r=>{if(p.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),i=r&&r.target&&r.target.src;a.message="Loading chunk "+t+" failed.\n("+o+": "+i+")",a.name="ChunkLoadError",a.type=o,a.request=i,n[1](a)}}),"chunk-"+t,t)}};var t=(t,r)=>{var n,o,[i,a,s]=r,c=0;if(i.some((t=>0!==e[t]))){for(n in a)p.o(a,n)&&(p.m[n]=a[n]);s&&s(p)}for(t&&t(r);c<i.length;c++)o=i[c],p.o(e,o)&&e[o]&&e[o][0](),e[o]=0},r=self.webpackChunkcreate_wasm_app=self.webpackChunkcreate_wasm_app||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))})(),s={},c={414:function(){return{"./wasm_fluid_bg.js":{__wbindgen_object_drop_ref:function(e){return void 0===r&&(r=p.c[799].exports),r.ug(e)},__wbg_buffer_085ec1f694018c4f:function(e){return void 0===n&&(n=p.c[799].exports),n.Zf(e)},__wbg_newwithbyteoffsetandlength_69193e31c844b792:function(e,t,r){return void 0===o&&(o=p.c[799].exports),o.Qk(e,t,r)},__wbindgen_throw:function(e,t){return void 0===i&&(i=p.c[799].exports),i.Or(e,t)},__wbindgen_memory:function(){return void 0===a&&(a=p.c[799].exports),a.oH()}}}}},u={170:[414]},p.w={},p.f.wasm=function(e,t){(u[e]||[]).forEach((function(r,n){var o=s[r];if(o)t.push(o);else{var i,a=c[r](),u=fetch(p.p+""+{170:{414:"dd1e8b3dd1b85b439b8d"}}[e][r]+".module.wasm");i=a&&"function"==typeof a.then&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(u),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(u,a):u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)})),t.push(s[r]=i.then((function(e){return p.w[r]=(e.instance||e).exports})))}}))},p(820)})();