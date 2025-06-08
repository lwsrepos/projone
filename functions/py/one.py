from pkg.state import AppState
import uuid


async def handle(state: AppState, req_id: str|None=None):
    return uuid.uuid4().hex.encode()
