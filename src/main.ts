import { enable, isEnabled } from "tauri-plugin-autostart-api";

enable().then(() => {});

// 1. Select the div element using the id property
const app = document.getElementById("app");
// 2. Create a new <p></p> element programmatically
const autostartIsEnabled = document.createElement("p");
// 3. Add the text content

autostartIsEnabled.textContent = "Hello, World!";

isEnabled().then((enabled) => {
  console.log(enabled);
});

// 4. Append the p element to the div element
app?.appendChild(autostartIsEnabled);
