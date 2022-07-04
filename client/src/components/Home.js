import { Button, Container, VStack } from "@chakra-ui/react";

export default function Home(props) {
    const { onLogin, onRegister } = props

    return (
        <Container>
            <VStack spacing={2}>
                <p>WebAuthn Demo</p>
                <Button variant="outlined"  >Login</Button>
                <Button colorScheme='blue'>Register</Button>
            </VStack>
        </Container>
    )
}