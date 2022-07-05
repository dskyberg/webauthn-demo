import { createContext, useContext, useState } from 'react'
import { useLocation, Navigate } from "react-router-dom";

const AuthContext = createContext(null)

export const AuthProvider = ({ children }) => {
    const [user, setUser] = useState(null);

    const signin = (user, callback = () => { }) => {
        setUser(user)
        callback()
    }

    const signout = (callback = () => { }) => {
        setUser(null)
        callback()
    }

    let value = { user, signin, signout };
    return (
        <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
    );
};

export function useAuth() {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error('useAuth must be used within an AuthProvider.');
    }
    return context;
};

export function RequireAuth(props) {
    const { children } = props
    let auth = useAuth();
    let location = useLocation();

    if (!auth.user) {
        // Redirect them to the /login page, but save the current location they were
        // trying to go to when they were redirected. This allows us to send them
        // along to that page after they login, which is a nicer user experience
        // than dropping them off on the home page.
        console.log("Auth Required for this page. Sending you to Login.")
        return <Navigate to="/login" state={{ from: location }} replace />;
    }

    return children;
}