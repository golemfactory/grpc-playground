import asyncio
import ssl
import sys

from grpclib.client import Channel

from helloworld_grpc import GreeterStub
from helloworld_pb2 import HelloRequest


def make_request(host, port, client_cert, client_key, server_cert):
    context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
    context.load_cert_chain(client_cert, client_key)
    context.load_verify_locations(server_cert)

    loop = asyncio.get_event_loop()
    channel = Channel(host, port, loop=loop, ssl=context)
    stub = GreeterStub(channel)

    request = HelloRequest(name='World')
    print('Sending request...')
    reply = loop.run_until_complete(stub.SayHello(request))
    print(f'Got reply: "{reply.message}"')

    channel.close()
    loop.close()


if __name__ == '__main__':
    make_request(*sys.argv[1:])
