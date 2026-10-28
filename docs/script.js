(function(){var t=1095,d=document.getElementById("d"),s=performance.now();
function f(n){var p=Math.min((n-s-300)/2400,1);if(p<0)p=0;d.textContent=Math.round(t*(1-Math.pow(1-p,3)));if(p<1)requestAnimationFrame(f)}requestAnimationFrame(f);
var b=document.getElementById("bars");new IntersectionObserver(function(e,o){if(e[0].isIntersecting){b.parentNode.classList.add("in");o.disconnect()}},{threshold:.4}).observe(b);
try{fetch("https://api.github.com/repos/Shariqtechie/Stasis").then(function(r){return r.json()}).then(function(j){if(j.stargazers_count!=null)document.getElementById("n").textContent="· "+j.stargazers_count}).catch(function(){})}catch(e){}})();
