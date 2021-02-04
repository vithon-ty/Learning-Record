from abc import ABC, ABCMeta, abstractclassmethod
from user import User
from username import UserName


class IUserRepository(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractclassmethod
    def save(self, user: User):
        pass

    @abstractclassmethod
    def find(self, name: UserName) -> User:
        pass

    @abstractclassmethod
    def exists(self, user: User) -> bool:
        pass
