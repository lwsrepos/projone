from pkg.state import AppState
import uuid


def handle(state: AppState, req_id: str|None=None):
    return uuid.uuid4().hex.decode()
