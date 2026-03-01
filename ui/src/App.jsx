import { BrowserRouter, Routes, Route } from "react-router-dom";

import Dashboard from './pages/dashboard/Dashboard.jsx';
import HomePage from './pages/home/HomePage.jsx';

export default function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path = "/" element={<HomePage />} />;
                <Route path = "/dashboard" element={<Dashboard />} />;
            </Routes>
        </BrowserRouter>
    )
}
