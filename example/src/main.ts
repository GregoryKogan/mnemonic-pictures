import './style.css'
import { greet, draw_circle } from 'mnemonic-pictures'


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>${greet("JavaScript")}</h1>
    <canvas id="displayCanvas" width="300" height="200"></canvas>
  </div>
`

draw_circle("displayCanvas");
