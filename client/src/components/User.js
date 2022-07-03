/** @jsxImportSource @emotion/react */
import React, { useState, useEffect } from 'react';
import { css } from '@emotion/react'

import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import { checkUser, getUserCredentials } from '../webauthn';
import AuthenticatorCard from './AuthenticatorCard';
import { PropaneSharp } from '@mui/icons-material';

export default function User(props) {
  const [authenticators, setAuthenticators] = useState([])
  const [user, setUser] = useState({ id: '', name: '', displayName: '' })
  const [credentials, setCredentials] = useState(null)

  useEffect(() => {
    console.log('Attempting to fetch: ', props.user)
    checkUser({ name: props.user })
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

  const logout = () => {
    props.onLogout()
  }

  const styles = {
    heading: css`
      margin: 1.2em;
      padding-top: 1.2em;
      padding-bottom: 1.2em;`,
  }

  const doCredentials = () => {
    return <React.Fragment>
      <Typography css={styles.heading} variant="subtitle">Credential</Typography>
      <Typography variant="body2">Type: {credentials.type}</Typography>
      <Typography variant="body2">AAGUID: {credentials.aaguid}</Typography>
      <Typography variant="body2">ID: {credentials.id}</Typography>
      <Typography variant="body2">{credentials.id}</Typography>
    </React.Fragment>

  }

  return (
    <Container maxWidth="sm">
      <Typography css={styles.heading} variant="subtitle">User</Typography>
      <Typography variant="body2">Name {user.name}</Typography>
      <Typography variant="body2">Display Name: {user.displayName}</Typography>
      <Typography variant="body2">ID: {user.id}</Typography>
      {
        credentials !== null && doCredentials()
      }
    </Container >
  )
}
/*
      <Typography variant="h6">Your authenticators:</Typography>
      <Button variant="primary" onClick={logout}>Log Out</Button>
      {
        authenticators.map(authenticator => <AuthenticatorCard key={authenticator.credID} authenticator={authenticator} />)
      }
*/