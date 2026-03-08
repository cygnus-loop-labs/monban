import { TrashIcon } from "@phosphor-icons/react";

export default function BlackListRow({ word, rank, onDelete }) {
    return (
        <div className="blacklist-row"
            onMouseEnter={e => e.currentTarget.style.background = "var(--border)"}
            onMouseLeave={e => e.currentTarget.style.background = "transparent"}
        >
            <span className="blacklist-row__index">
                {String(rank).padStart(2, "0")}
            </span>
            <span className="blacklist-row__word">
                {word.word}
            </span>
            <div className="blacklist-row__actions">
                <button className="blacklist-row__remove" onClick={() => onDelete(word)}>
                    <TrashIcon className="blacklist-row__trash"/>
                </button>
            </div>
        </div>
    );
}
