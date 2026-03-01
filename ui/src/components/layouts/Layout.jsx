import { Header } from "./Header";

export function Layout({ children }) {
    return (
        <div className="layout">
            <Header />
            { children }
        </div>
    )
}
