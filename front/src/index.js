import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
//import './styles.css';

import App from './App';


const domNode = document.getElementById('root');
const root = createRoot(domNode);
root.render(<App/>);