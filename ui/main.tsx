import * as React from 'react';
import * as ReactDOM from 'react-dom';

ReactDOM.render((
    <>
        Hello, World! { Math.floor(Math.random() * 100) }
    </>),
    document.getElementById("react-root"))
