import React, { useState, useEffect } from 'react';

import { Center, Container, Skeleton } from '@chakra-ui/react';
import { useAuth } from '../../auth'

import { getUser, getUserCredentials } from '../../webauthn';
import UserProfile from './UserProfile'
import UserCredential from './UserCredential'
import EmptyUser from './EmptyUser'

export default function User(props) {
    const auth = useAuth()
    const [user, setUser] = useState(null)
    const [credentials, setCredentials] = useState(null)

    useEffect(() => {
        if (!auth.isLoggedIn) {
            console.log('User.useEffect: not logged in.')
            return;
        }
        getUser({ name: auth.user })
            .then(user => {
                console.log('Found User:', user)
                setUser(user)
                return (user)
            })
            .then(user => {
                return getUserCredentials(user)
            })
            .then(credentials => {
                console.log('Got credentials:', credentials)
                setCredentials(credentials)
            })

    }, [])

    if (user === null) {
        return (
            <EmptyUser />
        )
    }

    return (
        <Center>
            <Container m="2rem" maxWidth="800">
                <UserProfile user={user} />
                <UserCredential credential={credentials} />
            </Container >
        </Center >
    )
}
