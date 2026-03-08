import { useEffect, useReducer } from "react";

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { createContext, useContext } from "react"

const initialState = {
    appState: {
        currentFile: null,
    },
    lexicon: {
        loading: false,
        progress: 0,
        error: null,
        data: null,
    },
    blacklist: {
        loading: false,
        progress: 0,
        error: null,
        data: null,
    }
};

const AppStateContext = createContext(initialState.appState);
const LexiconContext = createContext(initialState.lexicon);
const BlackListContext = createContext(initialState.blacklist);

function reducer(state, action) {
    switch (action.type) {
        case "ANALYZE_START":
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    loading: true, error: null, progress: 0
                }
            };

        case "ANALYZE_PROGRESS":
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    progress: action.progress
                }
            };

        case "ANALYZE_FINISHED":
            return {
                ...state,
                appState: { ...state.appState, currentFile: action.file },
                lexicon: { 
                    ...state.lexicon,
                    loading: false, error: null, progress: 100, data: action.lexicon
                },
            };

        case "ANALYZE_ERROR":
            console.log("analyse error: ", action.error);
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    loading: false, error: action.error, progress: 0, data: null
                },
            };

        case "BLACKLIST_LOAD_START":
            return {
                ...state,
                blacklist: {
                    ...state.blacklist,
                    loading: true, error: null, progress: 0
                }
            };

        case "BLACKLIST_LOAD_FINISHED":
            return {
                ...state,
                blacklist: {
                    ...state.blacklist,
                    loading: false, error: null, progress: 100, data: action.blacklist
                }
            };

        case "BLACKLIST_ADD_WORD":
            if (!state.lexicon.data || !state.blacklist.data) return state;
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    data: {
                        ...state.lexicon.data,
                        words: {
                            ...state.lexicon.data.words,
                            ...(state.lexicon.data.words[action.word.word] && {
                                [action.word.word]: {
                                    ...state.lexicon.data.words[action.word.word],
                                    filter: true
                                }
                            })
                        }
                    }
                },
                blacklist: {
                    ...state.blacklist,
                    data: [ ...state.blacklist.data, action.word ]
                }
            };

        case "BLACKLIST_REMOVE_WORD":
            if (!state.lexicon.data || !state.blacklist.data) return state;
            return {
                ...state,
                lexicon: {
                    ...state.lexicon,
                    data: {
                        ...state.lexicon.data,
                        words: {
                            ...state.lexicon.data.words,
                            ...(state.lexicon.data.words[action.word.word] && {
                                [action.word.word]: {
                                    ...state.lexicon.data.words[action.word.word],
                                    filter: false
                                }
                            })
                        }
                    }
                },
                blacklist: {
                    ...state.blacklist,
                    data: state.blacklist.data.filter(w => w.word !== action.word.word)
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

    async function get_blacklist() {
        dispatch({ type: "BLACKLIST_LOAD_START"});

        try {
            const blacklist = await invoke("get_blacklist");
            dispatch({ type: "BLACKLIST_LOAD_FINISHED", blacklist });
        } catch (e) {
            console.log("load blacklist error: ", e);
        } finally {

        }
    }

    function blacklist_add_word(word) {
        dispatch({ type: "BLACKLIST_ADD_WORD", word});
    }

    function blacklist_remove_word(word) {
        dispatch({ type: "BLACKLIST_REMOVE_WORD", word});
    }

    useEffect(() => {
        get_blacklist();
    }, []);


    const appState = {
        ...state.appState,
        analyze,
        blacklist_add_word,
        blacklist_remove_word,
    };

    return (
        <AppStateContext.Provider value={appState}>
            <LexiconContext.Provider value={state.lexicon}>
                <BlackListContext.Provider value={state.blacklist}>
                    { children }
                </BlackListContext.Provider>
            </LexiconContext.Provider>
        </AppStateContext.Provider>
    )
}

export const useAppState = () => useContext(AppStateContext);
export const useLexicon = () => useContext(LexiconContext);
export const useBlacklist = () => useContext(BlackListContext);
