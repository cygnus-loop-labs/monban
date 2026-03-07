import { ProgressBar } from "../../components/ui/ProgressBar";
import { Layout } from '../../components/layouts/Layout.jsx';

export default function LoadingScreen({ progress = null, message = null }) {
    return (
        <div className="text-serif-xxl loading-progress">
            Loading
            <ProgressBar height="20px" width="200px" value={progress} color="var(--red)" />
        </div>
    );
}
