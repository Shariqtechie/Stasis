const inv=(c,a)=>window.__TAURI__.core.invoke(c,a);
const P=[["7 days",7],["30 days",30],["90 days",90],["1 year",365],["3 years",1095],["5 years",1825],["Custom",0]];
let sel=1095;const $=i=>document.getElementById(i);
$("chips").innerHTML=P.map(([t,d])=>`<button class="chip${d==sel?" on":""}${d?"":" cust"}" data-d="${d}">${t}</button>`).join("");
$("chips").onclick=e=>{const b=e.target.closest(".chip");if(!b)return;sel=+b.dataset.d;
 document.querySelectorAll(".chip").forEach(c=>c.classList.toggle("on",c==b));$("date").hidden=sel!==0};
function toast(m,err){const t=$("toast");t.textContent=m;t.className="show"+(err?" err":"");setTimeout(()=>t.className="",3200)}
async function refresh(){
 const s=await inv("status");$("os").textContent=s.os;
 document.body.className=s.paused?"frozen":"live";
 $("state").textContent=s.paused?"Updates are frozen":"Updates are running";
 $("until").textContent=s.paused?"Until "+new Date(s.expiry).toLocaleDateString(undefined,{day:"numeric",month:"long",year:"numeric"}):"Nothing is paused yet";
 let n=0,t=Math.max(s.days_left,0);const el=$("days"),st=performance.now();
 (function f(now){const p=Math.min((now-st)/900,1);el.textContent=Math.round(t*(1-Math.pow(1-p,3)));if(p<1)requestAnimationFrame(f)})(st);
 const off=282.7*(1-Math.min(t/(s.total_days||1825),1));$("bar").style.strokeDashoffset=off;$("glow").style.strokeDashoffset=off;
 const b=await inv("backups");$("bk").hidden=!b.length;
 $("list").innerHTML=b.slice(0,2).map(n=>`<div class="row"><span>${n.replace("stasis_","").replace(".json","").replace("_"," ")}</span><button data-n="${n}">Restore</button></div>`).join("");
}
async function run(p){try{toast(await p);await refresh()}catch(e){toast(String(e),1)}}
$("go").onclick=()=>run(sel?inv("apply",{days:sel}):inv("apply",{date:$("date").value}));
$("resume").onclick=()=>run(inv("resume"));
$("list").onclick=e=>{const n=e.target.dataset.n;if(n)run(inv("restore",{name:n}))};
refresh().catch(e=>toast(String(e),1));
const W=window.__TAURI__.window.getCurrentWindow();
$("min").onclick=()=>W.minimize();$("cls").onclick=()=>W.close();
inv("mica").then(m=>document.documentElement.classList.toggle("mica",m)).catch(()=>{});
