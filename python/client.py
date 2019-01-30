import asyncio

from grpclib.client import Channel

from python.helloworld_grpc import GreeterStub
from python.helloworld_pb2 import HelloRequest


def make_request():
    loop = asyncio.get_event_loop()
    channel = Channel('127.0.0.1', 50051, loop=loop)
    stub = GreeterStub(channel)

    request = HelloRequest(name='World')
    print('Sending request...')
    reply = loop.run_until_complete(stub.SayHello(request))
    print(f'Got reply: "{reply.message}"')

    channel.close()
    loop.close()


if __name__ == '__main__':
    make_request()