import { invoke } from '@tauri-apps/api'
import { Component } from 'react'
import './App.css'

class App extends Component {
    constructor(props) {
        super(props);
        this.state = { spend: 0, greet: 'Woopos!' };
    }

    greet() {
        var start = new Date();
        invoke('greet', { name: 'World' }).then((response) => {
            var greet = response;
            var now = new Date();
            var spend = now - start;
            this.setState({greet, spend});
        })
    }

    render() {
        return (
        <div className="body">
            <button onClick={() => this.greet()}>Click me!</button>
            <div>{this.state.greet}</div>
            <div>This operation spend {this.state.spend}ms!</div>
        </div>
        );
    }
}

export default App
