import { createContext, useContext } from 'react'
import PolicyStore from './PolicyStore'

const StoreContext = createContext(null)

export const StoreProvider = ({ children }) => {

    const store = {
        policy: new PolicyStore(),
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