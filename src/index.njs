import htm from "https://esm.sh/htm";
import { h, render } from "https://esm.sh/preact";

// Initialize htm with Preact
const html = htm.bind(h);

function App(props) {
  return html` <div>
    ${props.cpus.map((cpu) => {
      return html`
        <div>
          <h3>${cpu.toFixed(1)}% usage</h3>
        </div>
      `;
    })}
  </div>`;
}

let i = 0;
setInterval(async () => {
  let response = await fetch("http://localhost:8081/api/cpus");

  if (response.status !== 200) {
    throw new Error(`${response.status}`);
  }

  let json = await response.json();
  const app = h("pre", null, JSON.stringify(json, null, 2));

  render(html`<${App} cpus=${json} />`, document.body);
}, 1000);
