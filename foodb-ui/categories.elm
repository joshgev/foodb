import Browser
import Html exposing (..)
import Html.Events exposing (..)
import Html.Attributes as Attributes
import Http
import Json.Decode as Decode
import Url.Builder as Url
import Debug
import Basics

main = Browser.element
    { init=init, update=update, subscriptions=subscriptions, view=view}

type alias Model = {categories: List String, message: String, newCategory: String}

init: () -> (Model, Cmd Msg)
init _ = 
    ((Model []) "nah" "", getCategories) 

type Msg = CreateCategory | GetCategories | Categories (Result Http.Error (List String))

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of
        GetCategories -> (model, Cmd.none)
        Categories result -> 
            case result of 
                Ok categories -> ({ model | categories = categories, message="yeah"}, Cmd.none)
                Err e -> ({model | message = (Debug.log "a" (Debug.toString  e))}, Cmd.none)
        CreateCategory -> ({model | message = "Created new category"}, Cmd.none)

subscriptions: Model -> Sub Msg
subscriptions model = Sub.none

view : Model -> Html Msg
view model = 
    div [] [
        text model.message,
        ul [] (List.map toLi model.categories),
        div [] [
            input [Attributes.placeholder "New Category", Attributes.value model.newCategory] [],
            button [onClick CreateCategory] [text "Add"]
        ]
    ]
toLi : String -> Html msg
toLi x = li [] [text x]

getCategories : Cmd Msg
getCategories = Http.send Categories (Http.get "http://localhost:8001/categories" categoriesDecoder)

categoriesDecoder : Decode.Decoder (List String)
categoriesDecoder = Decode.list (Decode.field "name" Decode.string)