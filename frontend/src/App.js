import './App.css';
import { useState, useEffect } from 'react';
import sizeof from 'object-sizeof';
import data from "./test.json";

function App() {

  const[wasm, setWasm] = useState(null);

  useEffect(async () => {
    setWasm(await import("to_bson"));
  }, []);

  if(wasm){
    const jsonData = {TIME: Date.now(), ...data};
    const bsonData = wasm.get_bson(jsonData);
    console.log("Size of json: ", sizeof(jsonData));
    console.log("Size of bson: ", sizeof(bsonData.bson));
  }
  return (
    <div className="App">
    </div>
  );
}

export default App;
