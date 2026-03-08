import { BrowserRouter, Routes, Route } from "react-router-dom";

import { AppProvider } from "./AppContext.jsx";
import Layout from './components/layouts/Layout.jsx';
import HomePage from './pages/home/HomePage.jsx';
import Dashboard from './pages/dashboard/Dashboard.jsx';
import BlackList from "./pages/blacklist/BlackList.jsx";

export default function App() {
    return (
        <AppProvider>
            <BrowserRouter>
                <Layout>
                    <Routes>
                        <Route path = "/" element={<HomePage />} />;
                        <Route path = "/dashboard" element={<Dashboard />} />;
                        <Route path = "/blacklist" element={<BlackList />} />;
                    </Routes>
                </Layout>
            </BrowserRouter>
        </AppProvider>
    )
}
