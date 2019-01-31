import asyncio

from grpclib.server import Server

from helloworld_grpc import GreeterBase
from helloworld_pb2 import HelloRequest, HelloReply


class Greeter(GreeterBase):

    async def SayHello(self, stream):
        request: HelloRequest = await stream.recv_message()
        print(f'Got message from {request.name}.')
        reply = HelloReply(message=f'Hello, {request.name}!')
        await stream.send_message(reply)


async def wakeup():
    try:
        while True:
            await asyncio.sleep(1)
    except asyncio.CancelledError:
        pass
    asyncio.get_event_loop().close()


def run_server():
    loop = asyncio.get_event_loop()
    greeter = Greeter()
    server = Server(handlers=[greeter], loop=loop)

    # Start server
    print('Starting server...')
    loop.run_until_complete(server.start('127.0.0.1', 54321))
    print('Started.')

    # Add wakeup coroutine cause asyncio can't handle Ctrl+C on Windows
    wakeup_task = loop.create_task(wakeup())

    # Run until closed
    try:
        loop.run_forever()
    except KeyboardInterrupt:
        print('Shutting down server...')
    finally:
        server.close()
        loop.run_until_complete(server.wait_closed())
        wakeup_task.cancel()


if __name__ == '__main__':
    run_server()
