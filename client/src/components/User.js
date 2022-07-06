import React, { useState, useEffect } from 'react';
import { useNavigate } from "react-router-dom";

import { Center, Container, Text, Box, Grid, GridItem, HStack, Checkbox } from '@chakra-ui/react';

import { checkUser, getUserCredentials } from '../webauthn';

const USER_PRESENT = 1;
const USER_VERIFIED = 4;
const ATTESTED_CREDENTIAL_DATA_INCLUDED = 64;
const EXTENSION_DATA_INCLUDED = 128;

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

  const showAlg = (alg) => {
    let txt
    switch (alg) {
      case -7: txt = 'ES256'; break
      case -35: txt = 'ES384'; break
      case -8: txt = 'EDDSA'; break
      default: txt = `Unknown: ${alg}`;
    }
    return (
      <Text>{txt}</Text>
    )
  }

  const showFlags = (flags) => {
    let userPresent = (flags & USER_PRESENT) > 0
    let userVerified = (flags & USER_VERIFIED) > 0
    let acData = (flags & ATTESTED_CREDENTIAL_DATA_INCLUDED) > 0
    let extData = (flags & EXTENSION_DATA_INCLUDED) > 0

    return (
      <HStack>
        <Text>{flags}</Text>
        <Checkbox readOnly isChecked={userPresent}>User Present</Checkbox>
        <Checkbox readOnly isChecked={userVerified}>User Verified</Checkbox>
        <Checkbox readOnly isChecked={acData}>Attested</Checkbox>
        <Checkbox readOnly isChecked={extData}>Extensions</Checkbox>
      </HStack>
    )
  }


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
        <GridItem colSpan={4}>{showAlg(credentials.credential_public_key.alg)}</GridItem>

        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Key</Text> </GridItem>
        <GridItem colSpan={4}><Text>{credentials.credential_public_key.bytes}</Text></GridItem>

        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Flags</Text> </GridItem>
        <GridItem colSpan={4}>{showFlags(credentials.flags)}</GridItem>

        <GridItem><Text as={'span'} fontWeight={'bold'} mr="1.2rem">Last Used</Text> </GridItem>
        <GridItem colSpan={4}><Text>{credentials.last}</Text></GridItem>
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
