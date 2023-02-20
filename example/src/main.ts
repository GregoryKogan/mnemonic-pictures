import './style.css'
import init, { greet, generate, init_console_errors } from '@gregorykogan/mnemonic-pictures';


await init();
init_console_errors();


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>${greet("JavaScript")}</h1>
    <canvas id="displayCanvas" width="600" height="400"></canvas>
  </div>
`

document.getElementById('displayCanvas')!.addEventListener(
  'click', 
  function() { 
    generate(
      "displayCanvas", 
      BigInt(Math.round(Math.random() * 1000000))
    ) 
  }, 
  false
);

generate(
  "displayCanvas", 
  BigInt(Math.round(Math.random() * 1000000)),
);
