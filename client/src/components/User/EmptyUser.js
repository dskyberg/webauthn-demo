import { Center, Container, Skeleton } from '@chakra-ui/react'

const EmptyUser = () => (
    <Center>
        <Container m="2rem" maxWidth="800">
            <Skeleton height="60px" mb="2rem" />
            <Skeleton height="60px" mt="2rem" />
        </Container >
    </Center >
)
export default EmptyUser