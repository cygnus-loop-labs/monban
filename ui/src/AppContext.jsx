import { useReducer } from "react";

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { createContext, useContext } from "react"

const initialState = {
    appState: {
        currentFile: null,
        loading: false,
        error: null,
        progress: 0,

    },
    lexicon: null,
};

const AppStateContext = createContext(initialState.appState);
const LexiconContext = createContext(initialState.lexicon);

function reducer(state, action) {
    switch (action.type) {
        case "ANALYZE_START":
            console.log("analyze start");
            return {
                ...state,
                appState: { ...state.appState, loading: true, error: null, progress: 0}
            };
        case "ANALYZE_PROGRESS":
            console.log("analyze progress");
            return {
                ...state,
                appState: { ...state.appState, progress: action.progress}
            };
        case "ANALYZE_FINISHED":
            console.log("analyze finished");
            return {
                ...state,
                appState: { ...state.appState, loading: false, currentFile: action.file, progress: 100 },
                lexicon: action.lexicon,
            };
        case "ANALYZE_ERROR":
            console.log("analyse error: ", action.error);
            return {
                ...state,
                appState: { ...state.appState, loading: false, error: action.error },
                lexicon: null
            };
        case "DELETE_WORD":
            console.log("delete word: ", action.word);
            if (!state.lexicon) return state;
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    words: {
                        ...state.lexicon.words,
                        [action.word.word]: {
                            ...state.lexicon.words[action.word.word],
                            filter: true
                        }
                    }
                }
            };

        default:
            return state;

    }
}

export function AppProvider({children}) {
    const [state, dispatch] = useReducer(reducer, initialState);

    async function analyze(path) {
        dispatch({ type: "ANALYZE_START"});

        const unlisten = await listen("progress", (event) => {
            dispatch({ type: "ANALYZE_PROGRESS", progress: event.payload});
        });

        try {
            const lexicon = await invoke("analyze", { input: path });
            dispatch({ type: "ANALYZE_FINISHED", lexicon, file: path });
        } catch (e) {
            console.log("analyze error: ", e);
            dispatch({ type: "ANALYZE_ERROR", error: e });
        } finally {
            unlisten();
        }
    }

    function deleteWord(word) {
        dispatch({ type: "DELETE_WORD", word});
    }

    const appState = {
        ...state.appState,
        analyze,
        deleteWord
    };

    return (
        <AppStateContext.Provider value={appState}>
            <LexiconContext.Provider value={state.lexicon}>
                { children }
            </LexiconContext.Provider>
        </AppStateContext.Provider>
    )
}

export const useAppState = () => useContext(AppStateContext);
export const useLexicon = () => useContext(LexiconContext);
