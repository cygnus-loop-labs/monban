import { Header } from "./Header.jsx";
import { Sidebar } from "./Sidebar.jsx";

export function Layout({ children }) {
    return (
        <div className="layout">
            <Header />
            <div className="layout__body">
                <Sidebar />
                <div className="layout__content">
                    { children }
                </div>
            </div>
        </div>
    )
}
