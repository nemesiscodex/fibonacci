import requests
import sys

import opentelemetry.instrumentation.requests

opentelemetry.instrumentation.requests.RequestsInstrumentor().instrument()

args = sys.argv[1:]

from opentelemetry import trace
from opentelemetry.exporter.otlp.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchExportSpanProcessor

# Resource can be required for some backends, e.g. Jaeger
# If resource wouldn't be set - traces wouldn't appears in Jaeger
resource = Resource(attributes={
    "service.name": "python-client"
})

trace.set_tracer_provider(TracerProvider(resource=resource))
tracer = trace.get_tracer(__name__)

otlp_exporter = OTLPSpanExporter(endpoint="localhost:4317", insecure=True)

span_processor = BatchExportSpanProcessor(otlp_exporter)

trace.get_tracer_provider().add_span_processor(span_processor)

with tracer.start_as_current_span("client-call") as span:

    if len(args) > 0:
        fib = int(args[0])
    else:
        fib = 5
    try:
        span.set_attribute("fib", str(fib))
        
        r = requests.get("http://localhost:3000/fib/" + str(fib))
        try:
            print(r.json())
        except:
            print(r.body)
    except:
        print("Error calling fib endpoint")