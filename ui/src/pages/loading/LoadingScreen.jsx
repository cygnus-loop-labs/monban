import { ProgressBar } from "../../components/ui/ProgressBar";

export default function LoadingScreen({ progress = null, message = null }) {
    return (
        <div className="loading">
            <span className="loading__label">Loading</span>
            <ProgressBar value={progress} width="240px" height="12px" color="var(--purple)" />
            <span className="loading__progress">{progress}%</span>
        </div>
    );
}
