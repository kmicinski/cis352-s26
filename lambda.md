---
layout: default
title: "Lambda Calculus Explorer"
permalink: /lambda
---

<style>
/* Full-page immersive layout — hide site chrome */
.lc-fullpage .wrapper { max-width: none; padding: 0; margin: 0; }
.lc-fullpage .page-content { padding: 0; margin: 0; }
.lc-fullpage .site-footer { display: none; }
.lc-fullpage .site-header { display: none; }

.lc-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 20px;
  background: rgba(255, 255, 255, 0.94);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  z-index: 10;
}

.lc-header h1 {
  margin: 0;
  font-family: 'JetBrains Mono', 'SF Mono', 'Menlo', monospace;
  font-size: 1.2em;
  font-weight: 700;
  color: #F76900;
  letter-spacing: -0.02em;
}

.lc-header .lc-back {
  font-size: 0.82em;
  color: #9ca3af;
  text-decoration: none;
  margin-left: auto;
  transition: color 0.2s;
}

.lc-header .lc-back:hover {
  color: #F76900;
}

.lc-frame {
  width: 100%;
  height: calc(100vh - 49px);
  border: none;
  display: block;
}
</style>

<div class="lc-header">
  <h1>&lambda;-Calculus Explorer</h1>
  <a class="lc-back" href="{{ site.baseurl | prepend: site.url }}/">&larr; Back to CIS352</a>
</div>
<iframe class="lc-frame" src="{{ site.baseurl }}/lambda-playground/www/index.html"></iframe>

<script>
// Apply full-page class to parent containers
document.body.parentElement.classList.add('lc-fullpage');
document.body.classList.add('lc-fullpage');
document.querySelectorAll('.page-content, .wrapper').forEach(function(el) {
  el.classList.add('lc-fullpage');
});
// Hide site header/footer
var header = document.querySelector('.site-header');
var footer = document.querySelector('.site-footer');
if (header) header.style.display = 'none';
if (footer) footer.style.display = 'none';
// Remove wrapper constraints
document.querySelectorAll('.page-content, .page-content > .wrapper').forEach(function(el) {
  el.style.maxWidth = 'none';
  el.style.padding = '0';
  el.style.margin = '0';
});
</script>
