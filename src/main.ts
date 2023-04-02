import { enable, isEnabled } from "tauri-plugin-autostart-api";

enable().then(() => {});

isEnabled().then((enabled) => {
  console.log(enabled);
});