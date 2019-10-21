from dataclasses import dataclass
from enum import Enum
from typing import Union, Optional


class ResourceType(Enum):
    CSV = "csv"
    JSON = "json"
    XML = "xml"
    Spreadsheet = "spreadsheet"
    NetCDF4 = "netcdf4"
    GeoTIFF = "geotiff"


@dataclass
class CSVProp:
    delimiter: str = ","


@dataclass
class Resource:
    id: str
    type: ResourceType
    prop: Optional[Union[CSVProp]] = None

    @staticmethod
    def deserialize(raw: dict):
        if raw['type'] == ResourceType.CSV.value and raw['prop'] is not None:
            prop = CSVProp(raw['prop']['delimiter'])
        else:
            prop = None
        return Resource(raw['id'], ResourceType(raw['type']), prop)