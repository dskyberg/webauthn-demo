import { createContext, useContext } from 'react'
import PolicyStore from './PolicyStore'
import UsersStore from './UsersStore'

const StoreContext = createContext(null)

export const StoreProvider = ({ children }) => {

    const store = {
        policy: new PolicyStore(),
        users: new UsersStore()
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