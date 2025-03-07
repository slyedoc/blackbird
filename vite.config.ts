import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
export default defineConfig({
  plugins: [
    tailwindcss(),
  ],
  
})

// /** @type {import('tailwindcss').Config} */
// module.exports = {
//     content: {
//       files: ["*.html", "./src/**/*.rs"],
//       transform: {
//         rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
//       },
//     },
//     theme: {
//       extend: {},
//     },
//     corePlugins: {
//       preflight: false,
//     },
//     plugins: [
//       require('@tailwindcss/forms'),
//     ],
//   }
  