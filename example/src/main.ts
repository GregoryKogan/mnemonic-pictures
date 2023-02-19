import './style.css'
import { greet, noise_fill } from '@gregorykogan/mnemonic-pictures';


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>${greet("JavaScript")}</h1>
    <canvas id="displayCanvas" width="100" height="65"></canvas>
  </div>
`

noise_fill(
  "displayCanvas", 
  BigInt(Math.round(Math.random() * 1000000)),
);
