import { useState, useEffect } from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import Client from '../webauthn/Client';
import AuthenticatorCard from './AuthenticatorCard';

export default function User(props) {

  const [authenticators, setAuthenticators] = useState([])

  useEffect(() => {


    fetch('authenticators', {
      method: 'GET',
      credentials: 'include',
    }).then(response => {
      if (response.status !== 200) {
        console.error(response.message)
        return
      }
      return response.json()
    }).then(authenticators => {
      console.log('Authenticators:', authenticators)
      setAuthenticators(authenticators)
    })
  }, [])

  const logout = () => {
    (new Client()).logout().then(() => props.onLogout())
  }

  return (
    <Container maxWidth="sm">
      <Typography variant="h6">Welcome {props.user.name}</Typography>
      <Typography variant="h6">Your authenticators:</Typography>
      <Button variant="primary" onClick={logout}>Log Out</Button>
      {
        authenticators.map(authenticator => <AuthenticatorCard key={authenticator.credID} authenticator={authenticator} />)
      }
    </Container >
  )
}