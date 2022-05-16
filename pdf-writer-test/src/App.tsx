import React from "react";
import tree from '../pdf-writer-wasm/src/assets/tree.png';
import sky from '../pdf-writer-wasm/src/assets/sky.png';
import logo from '../pdf-writer-wasm/src/assets/logo.jpg';

import { useState } from 'react'
import './App.css'
import init, { WasmApp } from 'pdf-writer-wasm';


function App() {
  const [app, setApp] = useState<WasmApp | undefined>();

  React.useEffect(() => {
    init().then(() => {
      console.log('init wasm-pack');
      const app = new WasmApp();
      setApp(app);
    });
  }, [])


  const createPdf = async (id: string) => {
    if (!app) {
      return;
    }
    let bytes = await WasmApp.get_pdf_image(id);
    downloadBlob(bytes, "test.pdf", "application/pdf");
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>Create some segments</p>
        <img id="tree" src={tree} alt={"tree"} width="200" onClick={() => createPdf("tree")}/>
        <img id="logo" src={logo} alt={"logo"} width="200" onClick={() => createPdf("logo")}/>
        <img id="sky" src={sky} alt={"sky"} width="200" onClick={() => createPdf("sky")}/>
      </header>
    </div>
  )
}


const downloadURL = (url: string, fileName: string) => {
  const a = document.createElement('a')
  a.href = url
  a.download = fileName
  document.body.appendChild(a)
  a.style.display = 'none'
  a.click()
  a.remove()
}

const downloadBlob = (data: Uint8Array, fileName: string, mimeType: string) => {
  const blob = new Blob([data], {
    type: mimeType
  })
  const url = window.URL.createObjectURL(blob)
  downloadURL(url, fileName)
  setTimeout(() => window.URL.revokeObjectURL(url), 1000)
}



export default App
