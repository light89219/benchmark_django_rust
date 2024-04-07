from django.urls import include, path
from rest_framework.routers import DefaultRouter

from .views import TodoViewSet

router = DefaultRouter()
router.register(r'todos', TodoViewSet)

urlpatterns = [
    path('', include(router.urls)),
]
