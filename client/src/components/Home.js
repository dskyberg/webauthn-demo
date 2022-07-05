import { useNavigate } from 'react-router-dom'

import { Button, Container, VStack, HStack, Center, Heading, Box, Text } from "@chakra-ui/react";

export default function Home(props) {
    let navigate = useNavigate()
    const { onLogin, onRegister } = props

    const handleClick = () => {
        navigate("/login", { replace: true })
    }

    return (
        <Container maxW={'3xl'}>
            <Center>
                <VStack spacing="2">
                    <Heading fontWeight={600}
                        fontSize={{ base: '2xl', sm: '4xl', md: '6xl' }}
                        lineHeight={'110%'}>
                        WebAuthn Demo
                    </Heading>
                    <Text>
                        WebAuthn is supported in the Chrome, Firefox, and Edge browsers to different degrees,
                        but support for credential creation and assertion using a U2F Token, like those provided
                        by Yubico and Feitian, is supported by all of them. The code for this demo can be found
                        here on GitHub. To read more about WebAuthn and what is does, check out webauthn.guide
                        for an introduction.
                    </Text>
                    <HStack spacing={4} >
                        <Box bg={"gray.100"} p={"1rem"} onClick={handleClick}>
                            <Text>Start your journey by registering to create a WebAuthn Credential.</Text>
                        </Box>
                        <Box bg={"gray.100"} p={"1rem"} onClick={handleClick}>
                            <Text>Continue your journey by authenticating with your credential.</Text>
                        </Box>
                    </HStack>
                </VStack>
            </Center>
        </Container>
    )
}