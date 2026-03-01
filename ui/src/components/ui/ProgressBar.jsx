export function ProgressBar({ height="5px", width="60px", value, color }) {
    return (
        <div className="progress-bar" style={{ height: height, width: width }} >
            <div className="progress-bar__content" style={{ width: `${value}%`, background: color }} />
        </div>
    );
}

