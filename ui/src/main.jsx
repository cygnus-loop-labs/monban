import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './styles/common.css';
import './components/ui/ui.css';
import './components/layouts/layouts.css';
import './pages/pages.css';

import App from './App.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
