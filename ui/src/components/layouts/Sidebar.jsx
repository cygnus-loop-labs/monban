import { useNavigate } from "react-router-dom";

import { ChartBarIcon, FileTextIcon, TrashSimpleIcon, GearIcon } from "@phosphor-icons/react";

export default function Sidebar() {
    const navigate = useNavigate();

    return (
        <nav className="sidebar">
            <button className = "sidebar__item" onClick={() => navigate("/dashboard") }>
                <ChartBarIcon />
            </button>
            <button className = "sidebar__item" onClick={() => navigate("/") }>
                <FileTextIcon />
            </button>
            <button className = "sidebar__item" onClick={() => navigate("/blacklist") }>
                <TrashSimpleIcon />
            </button>
            <button className = "sidebar__item" onClick={() => navigate("/") }>
                <GearIcon />
            </button>
        </nav>
    );
}
