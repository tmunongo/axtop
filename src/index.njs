import { h, render } from "https://esm.sh/preact";

let i = 0;
setInterval(async () => {
  let response = await fetch("http://localhost:8081/api/cpus");

  if (response.status !== 200) {
    throw new Error(`${response.status}`);
  }

  let json = await response.json();
  const app = h("pre", null, JSON.stringify(json, null, 2));

  render(app, document.body);
}, 1000);
