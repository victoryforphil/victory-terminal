import React, {useState, useEffect, useContext} from "react"

export const SerialContext = React.createContext({
    baudRate: 115200,
    output: [],
    isConnected: false,
    port: "COM3"
});


const SerialComponent = (props) =>{
    const ctx = useContext(SerialContext);
    const SerialPort = require("serialport");
    const [isOpened, setOpened] = useState(false);
    const [port, setPort] = useState(ctx.port)
    let sPort = undefined;
    
      
    useEffect(()=>{
        if(!isOpened){
            sPort = new SerialPort(port, {baudRate: ctx.baudRate});
            sPort.on('data', function (data) {
                console.log('Data:', data)
              })
        }
    },[isOpened])

    return (
        <SerialContext.Provider value={{
            baudRate, output,isConnected,port
        }}>
            {props.children}
        </SerialContext.Provider>
    )
}


export default SerialComponent;