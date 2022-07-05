import React, { useState, useEffect } from 'react';
import { useNavigate } from "react-router-dom";

import { Center, Container, Text, Box, Grid, GridItem, List, ListItem } from '@chakra-ui/react';

import { checkUser, getUserCredentials } from '../webauthn';

export default function User(props) {
  let navigate = useNavigate()
  const [authenticators, setAuthenticators] = useState([])
  const [user, setUser] = useState({ id: '', name: '', displayName: '' })
  const [credentials, setCredentials] = useState(null)

  useEffect(() => {
    /*   if (props.user === undefined || props.user === '') {
         console.log("User: no user name provided.  Redirecting to /")
         navigate("/", { replace: true })
       } else {
   */
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
    //  }
  }, [])

  const logout = () => {
    props.onLogout()
  }

  const showPublicKey = () => (
    <Box m="2.0rm">
      <Text
        fontSize={{ base: '16px', lg: '18px' }}
        color={'yellow.500'}
        fontWeight={'500'}
        textTransform={'uppercase'}
        mb={'2'}
      >Public Key</Text>
      <Grid templateColumns='repeat(5, 1fr)' spacing={2}>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Algorithm</Text></GridItem>
        <GridItem colSpan={4}><Text>{credentials.credential_public_key.alg}</Text></GridItem>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Key</Text> </GridItem>
        <GridItem colSpan={4}><Text>{credentials.credential_public_key.bytes}</Text></GridItem>
      </Grid>
    </Box>

  )

  const doCredentials = () => (
    <Box m="2.0rm">
      <Text
        fontSize={{ base: '16px', lg: '18px' }}
        color={'yellow.500'}
        fontWeight={'500'}
        textTransform={'uppercase'}
        mb={'2'}
      >Credential</Text>
      <Grid templateColumns='repeat(5, 1fr)' spacing={2}>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Type</Text></GridItem>
        <GridItem colSpan={4}><Text>{credentials.type}</Text></GridItem>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">AAGUID:</Text> </GridItem>
        <GridItem colSpan={4}><Text>{credentials.aaguid}</Text></GridItem>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">ID:</Text></GridItem>
        <GridItem colSpan={4}><Text>{credentials.id}</Text></GridItem>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Algorithm</Text></GridItem>
        <GridItem colSpan={4}><Text>{credentials.credential_public_key.alg}</Text></GridItem>
        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Key</Text> </GridItem>
        <GridItem colSpan={4}><Text>{credentials.credential_public_key.bytes}</Text></GridItem>
      </Grid>
    </Box>
  )


  return (
    <Center>
      <Container m="2rem" maxWidth="800">
        <Box mb="2rem">
          <Text
            fontSize={{ base: '16px', lg: '18px' }}
            color={'yellow.500'}
            fontWeight={'500'}
            textTransform={'uppercase'}
            mb={'2'}
          >User</Text>
          <Grid templateColumns='repeat(5, 1fr)' spacing={2}>
            <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Name </Text></GridItem>
            <GridItem colSpan={4}> <Text as={'span'} >{user.name}</Text></GridItem>

            <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Display Name </Text></GridItem>
            <GridItem colSpan={4}><Text as={'span'} >{user.displayName}</Text></GridItem>

            <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">ID </Text></GridItem>
            <GridItem colSpan={4}><Text as={'span'} >{user.id}</Text></GridItem>
          </Grid>
        </Box>
        {
          credentials !== null && doCredentials()
        }
      </Container >
    </Center >
  )
}
