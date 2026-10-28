const inv=(c,a)=>window.__TAURI__.core.invoke(c,a);
const REPO="https://github.com/Shariqtechie/Stasis";
const P=[["7 days",7],["30 days",30],["90 days",90],["1 year",365],["3 years",1095],["5 years",1825],["Custom",0]];
let sel=1095,ext=false,S=null;const $=i=>document.getElementById(i);
$("chips").innerHTML=P.map(([t,d])=>`<button class="chip${d==sel?" on":""}${d?"":" cust"}" data-d="${d}">${t}</button>`).join("");
$("chips").onclick=e=>{const b=e.target.closest(".chip");if(!b)return;sel=+b.dataset.d;
 document.querySelectorAll("#chips .chip").forEach(c=>c.classList.toggle("on",c==b));$("date").hidden=sel!==0};
function toast(m,err){const t=$("toast");t.textContent=m;t.className="show"+(err?" err":"");setTimeout(()=>t.className="",3200)}
const fmt=n=>{const m=n.match(/(\d{4})(\d\d)(\d\d)_(\d\d)(\d\d)(\d\d)/);if(!m)return n;
 return new Date(+m[1],m[2]-1,+m[3],+m[4],+m[5],+m[6]).toLocaleString(undefined,{day:"numeric",month:"short",year:"numeric",hour:"numeric",minute:"2-digit"})};
function render(){
 const s=S,mode=!s.paused?"none":s.by_app?"app":"win";
 document.body.className=mode=="app"?"frozen":mode=="win"?"win":"live";
 $("os").textContent=s.os;
 $("state").textContent={none:"Updates are running",app:"Updates are frozen",win:"Paused in Windows Settings"}[mode];
 $("until").textContent=s.paused?"Until "+new Date(s.expiry).toLocaleDateString(undefined,{day:"numeric",month:"long",year:"numeric"}):"Nothing is paused yet";
 $("unit").textContent=mode=="win"?"days paused":"days frozen";
 $("info").hidden=mode!="win";
 $("pick").hidden=mode=="app"&&!ext;$("go").hidden=$("pick").hidden;
 $("plabel").textContent=ext?"Extend freeze by":"Freeze updates for";
 $("go").textContent=ext?"Extend freeze":"Freeze updates";
 $("ext").hidden=mode!="app";$("ext").textContent=ext?"Cancel":"Extend freeze";
 $("resume").hidden=mode=="none";
}
async function refresh(){
 S=await inv("status");ext=false;render();
 const t=Math.max(S.days_left,0),el=$("days"),st=performance.now();
 (function f(now){const p=Math.min((now-st)/900,1);el.textContent=Math.round(t*(1-Math.pow(1-p,3)));if(p<1)requestAnimationFrame(f)})(st);
 const off=282.7*(1-Math.min(t/(S.total_days||1825),1));$("bar").style.strokeDashoffset=off;$("glow").style.strokeDashoffset=off;
 const b=await inv("backups");$("bk").hidden=!b.length;
 $("list").innerHTML=b.slice(0,2).map(n=>`<div class="row"><span>${fmt(n)}</span><button data-n="${n}">Restore</button></div>`).join("");
}
async function run(p){try{toast(await p);await refresh()}catch(e){toast(String(e),1)}}
$("go").onclick=()=>run(sel?inv("apply",{days:sel,extend:ext}):inv("apply",{date:$("date").value,extend:ext}));
$("ext").onclick=()=>{ext=!ext;render()};
$("resume").onclick=()=>run(inv("resume"));
$("list").onclick=e=>{const n=e.target.dataset.n;if(n)run(inv("restore",{name:n}))};
// About panel
const closeAbout=()=>$("about").hidden=true;
$("abt").onclick=()=>$("about").hidden=false;$("closeab").onclick=closeAbout;
$("about").onclick=e=>{if(e.target===$("about"))return closeAbout();const u=e.target.closest("[data-u]");if(u)inv("open_url",{url:u.dataset.u}).catch(x=>toast(String(x),1))};
document.addEventListener("keydown",e=>{if(e.key==="Escape")closeAbout()});
$("share").onclick=()=>inv("open_url",{url:"https://x.com/intent/post?text="+encodeURIComponent("Stasis freezes Windows Update for up to 5 years. Free and open source.")+"&url="+encodeURIComponent(REPO)}).catch(x=>toast(String(x),1));
$("copy").onclick=()=>navigator.clipboard.writeText(REPO).then(()=>toast("Link copied")).catch(()=>toast(REPO));
inv("version").then(v=>$("ver").textContent="v"+v).catch(()=>{});
refresh().catch(e=>toast(String(e),1));
const W=window.__TAURI__.window.getCurrentWindow();
$("min").onclick=()=>W.minimize();$("cls").onclick=()=>W.close();
inv("mica").then(m=>document.documentElement.classList.toggle("mica",m)).catch(()=>{});
