import { BrowserRouter, Routes, Route } from "react-router-dom";

import { Layout } from './components/layouts/Layout.jsx';
import Dashboard from './pages/dashboard/Dashboard.jsx';
import HomePage from './pages/home/HomePage.jsx';
import { AppProvider } from "./AppContext.jsx";

export default function App() {
    return (
        <AppProvider>
            <BrowserRouter>
                <Layout>
                    <Routes>
                        <Route path = "/" element={<HomePage />} />;
                        <Route path = "/dashboard" element={<Dashboard />} />;
                    </Routes>
                </Layout>
            </BrowserRouter>
        </AppProvider>
    )
}
