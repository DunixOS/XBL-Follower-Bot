import asyncio
import aiohttp
import json
from colorama import init
from sys import platform
from os import system, _exit, path
from typing import Dict, Optional
from base64 import b64encode
from urllib.parse import urlencode


class Follow_Bot:
    def __init__(self):
        self.users = []
        self.followed = 0
        self.failed = 0
        self.target = ''
        self.expired_tokens = []
        self.xbox_tokens: Dict[str, Optional[str]] = {}
        self.token_limit = None


    async def get_xbl_token(self, session: aiohttp.ClientSession, ms_token: str, token_id: int) -> Optional[str]:
        try:
            async with session.post('https://xsts.auth.xboxlive.com/xsts/authorize', json={
                "Properties": {
                    "SandboxId": "RETAIL",
                    "UserTokens": [ms_token]
                },
                "RelyingParty": "http://xboxlive.com",
                "TokenType": "JWT"
            }, headers={"Content-Type": "application/json", "Accept": "application/json"}) as response:
                if response.status != 200:
                    error_text = await response.text()
                    print(f"\n [\x1b[1;31m!\x1b[1;37m] Xbox Live Auth error (Status {response.status}): {error_text}")
                    if response.status != 200:
                        error_text = await response.text()
                        print(f"\n [\x1b[1;31m!\x1b[1;37m] XSTS error (Status {response.status}): {error_text}")
                        if response.status == 401 and '"XErr":2148916233' in error_text:
                            print(f" [\x1b[1;33m!\x1b[1;37m] Token {token_id} has expired and will be removed")
                            self.expired_tokens.append(ms_token)
                        return None

                xsts_data = await response.json()
                uhs = xsts_data.get('DisplayClaims', {}).get('xui', [{}])[0].get('uhs')
                xsts_token = xsts_data.get('Token')

                if not uhs or not xsts_token:
                    print(f"\n [\x1b[1;31m!\x1b[1;37m] Missing data in XSTS response")
                    return None

                return f"XBL3.0 x={uhs};{xsts_token}"
        except Exception as e:
            print(f"\n [\x1b[1;31m!\x1b[1;37m] Auth error: {str(e)}")
            return None

    async def follow_target(self, token: str) -> None:
        async with aiohttp.ClientSession() as session:
            try:
                token = token.strip()
                token_id = len(self.xbox_tokens) + 1
                if token not in self.xbox_tokens and token not in self.expired_tokens:
                    print(f"\n [\x1b[1;33m*\x1b[1;37m] Authorizing token {token_id} with XSTS...")
                    self.xbox_tokens[token] = await self.get_xbl_token(session, token, token_id)

                xbl_token = self.xbox_tokens[token]
                if not xbl_token:
                    self.failed += 1
                    print(f"\n [\x1b[1;31m!\x1b[1;37m] Failed to get Xbox Live token")
                    return

                async with session.put(
                    f'https://social.xboxlive.com/users/me/people/gt({self.target})',
                    headers={"Authorization": xbl_token, "X-XBL-Contract-Version": "2"}
                ) as follow_request:
                    if follow_request.status in [200, 201, 202, 204]:
                        self.followed += 1
                    else:
                        self.failed += 1
                        error_text = await follow_request.text()
                        print(f"\n [\x1b[1;31m!\x1b[1;37m] Failed with status {follow_request.status}: {error_text}")

                    print(f" [\x1b[1;32m+\x1b[1;37m] target: ({self.target}) | followed: ({self.followed}) | failed: ({self.failed})", end='\r', flush=True)
            except Exception as e:
                self.failed += 1
                print(f"\n [\x1b[1;31m!\x1b[1;37m] Request error: {str(e)}")


    def collect_tokens(self) -> list[str]:
        script_dir = path.dirname(path.abspath(__file__))
        tokens_path = path.join(script_dir, 'tokens.txt')
        with open(tokens_path, 'r') as token_file:
            all_tokens = [token.strip() for token in token_file]

        if self.token_limit and self.token_limit < len(all_tokens):
            return all_tokens[:self.token_limit]
        return all_tokens


    async def initialise(self) -> None:
        system('cls' if platform == 'win32' else 'clear')
        init(autoreset=True)
        print(' [\x1b[1;32m*\x1b[39m] cute wittle xbox follower bot')

        script_dir = path.dirname(path.abspath(__file__))
        tokens_path = path.join(script_dir, 'tokens.txt')
        if len(open(tokens_path, 'r').readlines()) > 0:
            print(f" [\x1b[1;32m*\x1b[39m] Tokens: ({len(open(tokens_path, 'r').readlines())}) \n")
        else:
            print(f' [\x1b[1;31m!\x1b[39m] no tokens found in \'\x1b[1;33m{tokens_path}\x1b[39m\'');_exit(0)

        total_tokens = len(open(tokens_path, 'r').readlines())
        token_input = input(f' [\x1b[1;32m?\x1b[39m] How many tokens to use? (1-{total_tokens}, press Enter for all): ').strip()

        if token_input:
            try:
                self.token_limit = min(int(token_input), total_tokens)
                if self.token_limit < 1:
                    self.token_limit = total_tokens
            except ValueError:
                self.token_limit = total_tokens

        self.users = self.collect_tokens()
        print(f" [\x1b[1;32m*\x1b[39m] Using {len(self.users)} tokens")

        self.target = input(' [\x1b[1;32m?\x1b[39m] target: ');print('')
        await self.start()


    async def start(self) -> None:
        await asyncio.gather(*[self.follow_target(user) for user in self.users if user not in self.expired_tokens])
        print(f"\n [\x1b[1;32m✓\x1b[1;37m] Finished! Target: {self.target} | Successfully followed: {self.followed} | Failed: {self.failed}", flush=True)
        if self.followed > 0:
            print(f" [\x1b[1;32m+\x1b[1;37m] Successfully sent {self.followed} follows to {self.target}!")
        if self.expired_tokens:
            valid_tokens = [token for token in self.users if token not in self.expired_tokens]
            script_dir = path.dirname(path.abspath(__file__))
            tokens_path = path.join(script_dir, 'tokens.txt')
            with open(tokens_path, 'w') as f:
                f.write('\n'.join(valid_tokens))
            print(f" [\x1b[1;33m*\x1b[1;37m] Removed {len(self.expired_tokens)} expired tokens from tokens.txt")


if __name__ == "__main__":
    asyncio.run(Follow_Bot().initialise())
