import { createContext, useContext } from 'react'
import JournalStore from './JournalStore'
import SettingsStore from './SettingsStore'

const StoreContext = createContext(null)

export const StoreProvider = ({ children }) => {

    const store = {
        journal: new JournalStore(),
        settings: new SettingsStore(),
    }

    return (
        <StoreContext.Provider value={store}>{children}</StoreContext.Provider>
    );
};

export const useStore = () => {
    const store = useContext(StoreContext);
    if (!store) {
        throw new Error('useStore must be used within a StoreProvider.');
    }
    return store;
};